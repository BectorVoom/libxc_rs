//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 676/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk676<F: Float>(t3788: F, t860: F, t1109: F, t5: F, t337: F, t2121: F, t3116: F, t3128: F, t3180: F, t3703: F, t858: F, t2210: F) -> (F, F, F, F, F, F, F) {
    let t3790 = t3788 * t860 / F::cast_from(96.0_f64);
    let t3791 = t5 * t1109;
    let t3792 = t337 * t3791;
    let t3793 = t2121 * t3792;
    let t3795 = t3116 * t3793 / F::cast_from(96.0_f64);
    let t3797 = t3128 * t3180 / F::cast_from(24.0_f64);
    let t3798 = t858 * t3703;
    let t3799 = t2210 * t3798;
    (t3790, t3791, t3792, t3793, t3795, t3797, t3799)
}
