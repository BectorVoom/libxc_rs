//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1634/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1634(t14753: f64, t4908: f64, t14744: f64, t11588: f64, t1714: f64, t3451: f64, t3447: f64, t14818: f64, t14781: f64, t14710: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11487: f64, t14713: f64, t14766: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64) -> (f64, f64, f64, f64) {
    let t15332 = t4908 * t14753;
    let t15335 = t4908 * t14744;
    let t15338 = t11588 * t1714;
    let t15339 = t15338 * t3451;
    let t15341 = 0.18518518518518518518e-3_f64 * t3447 * t15339;
    let t15347 = 2.0_f64 / 27.0_f64 * t14818;
    let t15348 = 4.0_f64 / 9.0_f64 * t14781;
    let t15349 = 2.0_f64 / 9.0_f64 * t14710;
    let t15357 = t11487 - 10.0_f64 / 27.0_f64 * t11211 - t11213 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t11215 + t11217 / 9.0_f64 - 5.0_f64 / 27.0_f64 * t14766 - t15347 + t15348 + t15349 - 2.0_f64 / 27.0_f64 * t14779 + t14790 / 3.0_f64 + t14784 / 9.0_f64 + t14787 / 18.0_f64 - t14799 - 2.0_f64 / 3.0_f64 * t14793 - t14796 / 3.0_f64 - t14713 / 6.0_f64;
    (t15332, t15335, t15341, t15357)
}
