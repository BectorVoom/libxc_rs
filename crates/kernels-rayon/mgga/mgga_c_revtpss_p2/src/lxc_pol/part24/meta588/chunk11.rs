//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1847/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1847(t118: f64, t1312: f64, t1502: f64, t1518: f64, t18245: f64, t1843: f64, t1847: f64, t1911: f64, t22633: f64, t22747: f64, t22758: f64, t23094: f64, t25043: f64, t25045: f64, t30138: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t6773: f64, t6934: f64, t75941: f64, t7889: f64, t87051: f64, t87064: f64, t87227: f64, t87237: f64, t89771: f64, t91789: f64, t92446: f64, t92453: f64, t92465: f64, t92466: f64, t92469: f64, t92490: f64, t92500: f64, t92504: f64, t93: f64, t94: f64) -> f64 {
    let t92516 = (2.0_f64 * t1312 * t87051 + 8.0_f64 * t1518 * t75941 + 12.0_f64 * t18245 * t5920 + 8.0_f64 * t22633 * t4248 + 8.0_f64 * t22633 * t7889 + 24.0_f64 * t30138 * t5920 + 6.0_f64 * t87237 * t93 + 12.0_f64 * t87064 + t87227) * t569 - 6.0_f64 * t94 * t87237 * t508 - 4.0_f64 * t1502 * t25043 - 4.0_f64 * t22747 * t1843 - 6.0_f64 * t5877 * t6765 + 6.0_f64 * t6773 * t6934 - t87227 * t508 + 4.0_f64 * t22758 * t1911 - t118 * (t89771 + t91789) + 4.0_f64 * t1847 * t23094 + t511 * (t92446 + t92453 + t92465 + t92466 + t92469 + t92490 + t92500 + t92504) - 8.0_f64 * t651 * t25043 * t1518 - 24.0_f64 * t4248 * t25045 - 12.0_f64 * t18245 * t5921;
    t92516
}
