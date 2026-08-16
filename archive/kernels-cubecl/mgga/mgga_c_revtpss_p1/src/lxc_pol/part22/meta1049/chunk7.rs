//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3693/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3693<F: Float>(t3568: F, t5819: F, t17209: F, t17569: F, t20824: F, t3172: F, t3711: F, t20879: F, t1042: F, t1214: F, t12956: F, t17232: F, t17412: F, t17505: F, t17541: F, t17552: F, t18281: F, t21184: F, t5296: F, t5302: F, t5304: F, t5381: F, t5384: F, t56796: F, t56798: F, t56812: F) -> (F, F) {
    let t69875 = t5819 * t3568;
    let t69885 = t17569 * t17209;
    let t69890 = t3711 * t3172 * t20824;
    let t69899 = t3711 * t3172 * t20879;
    let t69901 = F::cast_from(0.19055119163586549765e-3_f64) * t56796 + F::cast_from(0.3811023832717309953e-3_f64) * t56798 - F::cast_from(0.3811023832717309953e-3_f64) * t56812 + F::cast_from(0.28582678745379824648e-3_f64) * t12956 * t21184 + F::cast_from(0.47637797908966374414e-3_f64) * t5384 * t1042 * t5302 * t69875 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t5296 * t18281 * t1214 + F::cast_from(0.3811023832717309953e-3_f64) * t69885 - F::cast_from(0.11433071498151929859e-2_f64) * t5381 * t17232 - F::cast_from(0.31758531939310916276e-3_f64) * t69890 - F::cast_from(0.5081365110289746604e-2_f64) * t17412 * t5304 - F::cast_from(0.15244095330869239812e-2_f64) * t17505 * t17541 + F::cast_from(0.28582678745379824648e-2_f64) * t5381 * t17552 + F::cast_from(0.3811023832717309953e-3_f64) * t69899;
    (t69875, t69901)
}
