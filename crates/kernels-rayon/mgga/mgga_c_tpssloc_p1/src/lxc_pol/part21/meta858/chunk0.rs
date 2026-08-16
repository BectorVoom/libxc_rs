//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3115/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115(t14961: f64, t4869: f64, t18915: f64, t3415: f64, t14858: f64, t4875: f64, t15838: f64, t19267: f64, t3633: f64, t4700: f64, t63280: f64, t64446: f64, t64447: f64, t64454: f64, t64456: f64, t64458: f64, t64460: f64, t64462: f64, t64464: f64) -> (f64, f64, f64, f64) {
    let t64466 = 0.46785788981077169656e1_f64 * t4869 * t14961;
    let t64470 = 0.11696447245269292414e1_f64 * t18915 * t3415;
    let t64472 = 0.46785788981077169656e1_f64 * t14858 * t4875;
    let t64473 = 8.0_f64 * t15838 * t4700 * t64447 - t19267 * t3633 * t4700 + t63280 + t64446 - t64454 - t64456 - t64458 - t64460 - t64462 - t64464 + t64466 + t64470 + t64472;
    (t64466, t64470, t64472, t64473)
}
