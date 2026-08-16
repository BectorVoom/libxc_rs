//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1041/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1041<F: Float>(t47784: F, t42934: F, t42937: F, t42940: F, t42943: F, t42948: F, t42954: F, t42961: F, t42963: F, t42967: F, t42970: F, t47587: F, t47594: F, t47597: F, t47602: F, t47605: F, t47607: F, t47610: F) -> (F, F) {
    let t50987 = F::cast_from(12.0_f64) * t47784;
    let t51000 = -F::cast_from(0.17090058289204942852e-2_f64) * t47587 - t42934 - t42937 - t42940 + t42943 + t42948 - t42954 - t42961 + F::cast_from(0.7690526230142224284e-2_f64) * t42963 + F::cast_from(0.64087718584518535698e-3_f64) * t47594 - F::cast_from(0.3845263115071112142e-2_f64) * t42967 - F::cast_from(0.1281754371690370714e-2_f64) * t42970 - F::cast_from(0.64087718584518535698e-3_f64) * t47597 - t47602 + t47605 - F::cast_from(0.1922631557535556071e-2_f64) * t47607 + F::cast_from(0.1281754371690370714e-2_f64) * t47610;
    (t50987, t51000)
}
