//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2557/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557<F: Float>(t55279: F, t3115: F, t42793: F, t4911: F, t11200: F, t380: F, t16088: F, t3057: F, t4930: F, t1071: F, t15669: F, t12050: F, t15907: F) -> (F, F, F, F, F, F, F) {
    let t55280 = F::cast_from(0.14291339372689912324e-3_f64) * t55279;
    let t55293 = t3115 * t42793 * t4911;
    let t55294 = F::cast_from(0.14291339372689912324e-3_f64) * t55293;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55413 = t3057 * t4930;
    let t55464 = t15669 * t1071;
    let t55499 = t15907 * t12050;
    (t55280, t55294, t55330, t55331, t55413, t55464, t55499)
}
