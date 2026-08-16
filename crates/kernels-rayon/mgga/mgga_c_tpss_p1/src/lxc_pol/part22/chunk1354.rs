//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1354/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1354(t10514: f64, t10552: f64, t10662: f64, t10667: f64, t10897: f64, t1692: f64, t1812: f64, t18728: f64, t18812: f64, t198: f64, t19818: f64, t20514: f64, t207: f64, t2133: f64, t2428: f64, t2433: f64, t2439: f64, t3552: f64, t35525: f64, t3683: f64, t5849: f64, t5853: f64, t62807: f64, t62829: f64, t6354: f64, t63844: f64, t64248: f64, t64296: f64, t64770: f64, t66299: f64, t66603: f64, t823: f64) -> f64 {
    let t66750 = t198 * t207 * t66603 * t823 - 3.0_f64 * t2439 * t5853 * t35525 + 12.0_f64 * t3552 * t5849 * t3683 + 2.0_f64 * t1692 * t18812 * t63844 + 3.0_f64 * t2439 * t1812 * t10552 - 6.0_f64 * t2439 * t20514 * t10514 + 2.0_f64 * t1692 * t66299 * t2433 + 12.0_f64 * t3552 * t1812 * t10662 + 6.0_f64 * t3552 * t1812 * t10667 - 6.0_f64 * t1692 * t62807 * t64248 + 3.0_f64 * t2439 * t6354 * t2133 - 3.0_f64 * t2439 * t5853 * t64296 - t1692 * t5853 * t10897 + 12.0_f64 * t18728 * t64770 + 4.0_f64 * t1692 * t62829 * t19818 - t1692 * t20514 * t2428;
    t66750
}
