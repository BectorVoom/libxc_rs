//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 897/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk897<F: Float>(t1621: F, t7785: F, t1620: F, t2637: F, t7136: F, t5312: F, t2825: F, t586: F, t593: F, t1037: F, t5470: F, t1627: F, t2593: F) -> (F, F, F, F, F, F) {
    let t7786 = t1621 * t7785;
    let t7788 = F::new(4.0) / F::new(15.0) * t1620 * t7786;
    let t7790 = F::new(8.0) / F::new(15.0) * t7136 * t2637;
    let t7792 = F::new(8.0) / F::new(15.0) * t5312 * t2637;
    let t7793 = t2825 * t586;
    let t7795 = F::new(8.0) / F::new(45.0) * t7793 * t593;
    let t7797 = F::new(4.0) / F::new(45.0) * t5470 * t1037;
    let t7799 = F::new(16.0) / F::new(45.0) * t1627 * t2593;
    (t7788, t7790, t7792, t7795, t7797, t7799)
}
