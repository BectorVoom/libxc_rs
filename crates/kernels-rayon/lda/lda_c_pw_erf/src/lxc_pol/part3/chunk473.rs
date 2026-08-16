//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 473/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk473(t1074: f64, t1060: f64, t1001: f64, t1036: f64, t1040: f64, t1053: f64, t1057: f64, t1079: f64, t1083: f64, t1087: f64, t1802: f64, t1805: f64, t997: f64) -> f64 {
    let t1888 = 0.0001831155503675316_f64 * t1074;
    let t1889 = 0.5848223397455204_f64 * t1060;
    let t1890 = -t1802 + t1040 - t997 + t1036 + t1805 - t1888 + t1083 - t1053 - t1057 - t1889 + t1079 + t1087 - t1001;
    t1890
}
