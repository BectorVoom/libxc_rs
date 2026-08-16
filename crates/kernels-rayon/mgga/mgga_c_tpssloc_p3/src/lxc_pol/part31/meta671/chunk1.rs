//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2002/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2002(t27982: f64, t7032: f64, t26959: f64, t7435: f64, t7432: f64, t91957: f64, t27966: f64, t23963: f64, t23975: f64, t26055: f64, t26090: f64, t26911: f64, t27961: f64, t27972: f64, t27976: f64, t7026: f64, t7782: f64, t84190: f64, t96403: f64, t96502: f64, t96506: f64) -> f64 {
    let t102215 = t27982 * t7032;
    let t102217 = t7435 * t26959;
    let t102219 = t91957 * t7432;
    let t102221 = t27966 * t7032;
    let t102223 = -10.0_f64 / 3.0_f64 * t26911 * t26090 - 4.0_f64 / 3.0_f64 * t26055 * t7782 + 10.0_f64 * t84190 * t27961 + 10.0_f64 * t23963 * t96403 - 10.0_f64 / 3.0_f64 * t23975 * t27972 - 10.0_f64 / 3.0_f64 * t7026 * t96502 - 10.0_f64 / 3.0_f64 * t7026 * t96506 - 5.0_f64 / 3.0_f64 * t23975 * t27976 + 16.0_f64 / 9.0_f64 * t102215 + 32.0_f64 / 9.0_f64 * t102217 + 80.0_f64 / 9.0_f64 * t102219 + 32.0_f64 / 9.0_f64 * t102221;
    t102223
}
