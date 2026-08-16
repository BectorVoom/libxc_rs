//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1414/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1414(t100945: f64, t100946: f64, t100949: f64, t100952: f64, t100960: f64, t107545: f64, t107588: f64, t1398: f64, t1852: f64, t1858: f64, t2023: f64, t2029: f64, t22431: f64, t22453: f64, t28869: f64, t28904: f64, t3: f64, t580: f64, t6471: f64, t6483: f64, t7759: f64, t7774: f64, t96348: f64) -> f64 {
    let tv4rho3sigma9 = t107545 * t3 * t580 + t107588 * t1398 + 3.0_f64 * t1852 * t28904 + 3.0_f64 * t1858 * t28869 + t2023 * t22453 + t2029 * t22431 + 3.0_f64 * t6471 * t7774 + 3.0_f64 * t6483 * t7759 + 3.0_f64 * t100945 + 3.0_f64 * t100946 + 6.0_f64 * t100949 + 3.0_f64 * t100952 + 6.0_f64 * t100960 + 3.0_f64 * t96348;
    tv4rho3sigma9
}
