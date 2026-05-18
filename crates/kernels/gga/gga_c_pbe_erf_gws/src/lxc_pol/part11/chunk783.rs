//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 783/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk783<F: Float>(t12716: F, t1827: F, t587: F, t3411: F, t7130: F, t10424: F, t950: F, t1821: F, t1820: F, t7580: F, t1033: F, t3555: F) -> (F, F, F, F, F, F, F, F) {
    let t12717 = t1827 * t12716;
    let t12719 = F::new(4.0) / F::new(15.0) * t587 * t12717;
    let t12721 = F::new(16.0) / F::new(15.0) * t7130 * t3411;
    let t12722 = t10424 * t950;
    let t12723 = t1821 * t12722;
    let t12725 = F::new(8.0) / F::new(15.0) * t1820 * t12723;
    let t12726 = F::new(8.0) / F::new(135.0) * t7580;
    let t12728 = F::new(2.0) / F::new(5.0) * t1033 * t3555;
    (t12717, t12719, t12721, t12722, t12723, t12725, t12726, t12728)
}
