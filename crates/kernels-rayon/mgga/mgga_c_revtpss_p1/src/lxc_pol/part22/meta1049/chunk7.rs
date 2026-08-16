//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3693/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3693(t3568: f64, t5819: f64, t17209: f64, t17569: f64, t20824: f64, t3172: f64, t3711: f64, t20879: f64, t1042: f64, t1214: f64, t12956: f64, t17232: f64, t17412: f64, t17505: f64, t17541: f64, t17552: f64, t18281: f64, t21184: f64, t5296: f64, t5302: f64, t5304: f64, t5381: f64, t5384: f64, t56796: f64, t56798: f64, t56812: f64) -> (f64, f64) {
    let t69875 = t5819 * t3568;
    let t69885 = t17569 * t17209;
    let t69890 = t3711 * t3172 * t20824;
    let t69899 = t3711 * t3172 * t20879;
    let t69901 = 0.19055119163586549765e-3_f64 * t56796 + 0.3811023832717309953e-3_f64 * t56798 - 0.3811023832717309953e-3_f64 * t56812 + 0.28582678745379824648e-3_f64 * t12956 * t21184 + 0.47637797908966374414e-3_f64 * t5384 * t1042 * t5302 * t69875 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t5296 * t18281 * t1214 + 0.3811023832717309953e-3_f64 * t69885 - 0.11433071498151929859e-2_f64 * t5381 * t17232 - 0.31758531939310916276e-3_f64 * t69890 - 0.5081365110289746604e-2_f64 * t17412 * t5304 - 0.15244095330869239812e-2_f64 * t17505 * t17541 + 0.28582678745379824648e-2_f64 * t5381 * t17552 + 0.3811023832717309953e-3_f64 * t69899;
    (t69875, t69901)
}
