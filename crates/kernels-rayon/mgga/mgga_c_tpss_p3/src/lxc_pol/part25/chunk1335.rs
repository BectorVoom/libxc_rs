//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1335/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1335(t116: f64, t21785: f64, t13119: f64, t1339: f64, t13798: f64, t16037: f64, t1663: f64, t1760: f64, t1796: f64, t19577: f64, t19620: f64, t20134: f64, t20137: f64, t20224: f64, t20322: f64, t20346: f64, t20368: f64, t21017: f64, t21253: f64, t21750: f64, t21790: f64, t21856: f64, t26848: f64, t3493: f64, t4478: f64, t5314: f64, t5706: f64, t5799: f64, t5939: f64, t624: f64, t6243: f64, t63042: f64, t6436: f64, t6437: f64, t646: f64, t65533: f64, t67541: f64, t68868: f64, t7383: f64) -> (f64, f64) {
    let t71308 = t21785 * t116;
    let t71343 = 12.0_f64 * t19620 * t7383 * t13798 - 2.0_f64 * t71308 * t646 + 2.0_f64 * t20322 * t1663 - t624 * t21750 - t5799 * t5314 - t1796 * t16037 + 6.0_f64 * t1760 * t63042 * t21017 - t21253 * t5939 + 12.0_f64 * t19620 * t26848 * t4478 + 12.0_f64 * t68868 * t20134 + 6.0_f64 * t6243 * t20137 + t5706 * t21856 - 2.0_f64 * t6243 * t20224 + 2.0_f64 * t19577 * t6437 - 2.0_f64 * t1760 * t6436 * t13119 - 6.0_f64 * t65533 * t20346 - 2.0_f64 * t5706 * t21790 - 4.0_f64 * t3493 * t20368 - 4.0_f64 * t67541 * t1339;
    (t71308, t71343)
}
