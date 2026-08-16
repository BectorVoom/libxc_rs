//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1262/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1262(t21854: f64, t509: f64, t1270: f64, t20226: f64, t6245: f64, t18686: f64, t21017: f64, t13627: f64, t1845: f64, t13955: f64, t118: f64, t1322: f64, t13565: f64, t1600: f64, t1760: f64, t1800: f64, t1830: f64, t1834: f64, t1846: f64, t21180: f64, t21253: f64, t21576: f64, t21750: f64, t21786: f64, t21790: f64, t4631: f64, t4641: f64, t4675: f64, t485: f64, t5463: f64, t5801: f64, t6243: f64, t626: f64, t6309: f64, t6399: f64, t6437: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21855 = t509 * t21854;
    let t21856 = t21855 * t1270;
    let t21858 = t20226 * t6245;
    let t21863 = t18686 * t21017;
    let t21868 = t1845 * t13627;
    let t21871 = t1845 * t13955;
    let t21877 = -t118 * t21750 - 2.0_f64 * t1322 * t6399 - 2.0_f64 * t13565 * t1800 - 2.0_f64 * t1600 * t6309 - 2.0_f64 * t1760 * t21790 + t1760 * t21856 + 6.0_f64 * t1760 * t21858 + 6.0_f64 * t1760 * t21863 + 2.0_f64 * t1760 * t21868 - t1760 * t21871 - 4.0_f64 * t1800 * t21180 - t1830 * t4631 + t1834 * t5463 + t1846 * t21253 - 4.0_f64 * t21576 * t626 - t21786 * t485 - 4.0_f64 * t4641 * t5801 - 2.0_f64 * t4675 * t5801 + 2.0_f64 * t6243 * t6437;
    (t21855, t21856, t21858, t21863, t21868, t21871, t21877)
}
