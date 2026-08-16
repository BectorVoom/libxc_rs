//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1232/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1232(t18999: f64, t509: f64, t1270: f64, t1845: f64, t9909: f64, t10456: f64, t118: f64, t1760: f64, t1796: f64, t1800: f64, t1830: f64, t1846: f64, t18544: f64, t18707: f64, t18711: f64, t18714: f64, t18717: f64, t18896: f64, t18898: f64, t18903: f64, t18919: f64, t18930: f64, t2054: f64, t2056: f64, t2106: f64, t3166: f64, t485: f64, t544: f64, t5706: f64, t5801: f64, t5809: f64, t5895: f64, t5937: f64, t624: f64, t626: f64, t646: f64, t7798: f64) -> (f64, f64, f64, f64) {
    let t19000 = t509 * t18999;
    let t19001 = t19000 * t1270;
    let t19005 = t1845 * t9909;
    let t19009 = -2.0_f64 * t5801 * t2106 - 4.0_f64 * t626 * t18707 + 6.0_f64 * t1760 * t18711 + 2.0_f64 * t1760 * t18714 + 3.0_f64 * t1760 * t18717 + t18544 * t1846 - t118 * t18896 - 4.0_f64 * t18898 * t646 + t18919 * t544 - t2054 * t1830 - 2.0_f64 * t624 * t5895 - 2.0_f64 * t7798 * t1800 - 4.0_f64 * t10456 * t1800 - 4.0_f64 * t2056 * t5809 - 4.0_f64 * t626 * t18930 - t1796 * t3166 + t1760 * t19001 + 2.0_f64 * t5706 * t5937 - t1760 * t19005 - 2.0_f64 * t18903 * t485;
    (t19000, t19001, t19005, t19009)
}
