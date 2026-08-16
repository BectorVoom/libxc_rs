//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3010/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010(t3040: f64, t5914: f64, t3166: f64, t5872: f64, t1023: f64, t11034: f64, t11054: f64, t11059: f64, t14596: f64, t14651: f64, t18080: f64, t18083: f64, t18088: f64, t18094: f64, t18099: f64, t18104: f64, t18111: f64, t18161: f64, t3186: f64, t3188: f64, t3200: f64, t3201: f64, t43470: f64, t43562: f64, t4649: f64, t4669: f64, t4673: f64, t4689: f64, t50509: f64, t50610: f64, t5932: f64) -> (f64, f64, f64) {
    let t62925 = t5914 * t3040;
    let t62945 = t3166 * t5872;
    let t62953 = 4.0_f64 * t1023 * t4649 * t50509 * t50610 + 4.0_f64 * t11054 * t3186 * t5932 + 12.0_f64 * t11059 * t18080 * t18111 + 8.0_f64 * t18088 * t3186 * t4673 + 4.0_f64 * t18099 * t3186 * t4673 + 4.0_f64 * t18161 * t3186 * t4673 + 2.0_f64 * t3186 * t3188 * t62925 - t3200 * t3201 * t62945 + 4.0_f64 * t11034 * t18083 + 2.0_f64 * t14596 * t4669 + 4.0_f64 * t14651 * t4689 + 2.0_f64 * t18094 * t43562 - 12.0_f64 * t18104 * t43470;
    (t62925, t62945, t62953)
}
