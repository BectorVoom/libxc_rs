//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 577/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk577(t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t342: f64, t381: f64, t4743: f64, t4857: f64, t4954: f64, t4961: f64, t4964: f64, t4967: f64, t4970: f64, t4977: f64, t4981: f64, t4984: f64, t4988: f64, t4992: f64, t4996: f64, t4999: f64, t5005: f64, t5009: f64, t5012: f64, t989: f64) -> f64 {
    let t5015 = 0.65854491829355115987e0_f64 * t4743 * t381 - 0.65854491829355115987e0_f64 * t4857 * t1083 + 0.65854491829355115987e0_f64 * t4954 * t1090 + 0.65854491829355115987e0_f64 * t1647 * t1093 - 0.65854491829355115987e0_f64 * t3223 * t1685 + 0.13170898365871023197e1_f64 * t3204 * t4961 - 0.65854491829355115987e0_f64 * t3287 * t4964 - 0.65854491829355115987e0_f64 * t1024 * t4967 - 0.65854491829355115987e0_f64 * t1024 * t4970 + 0.65854491829355115987e0_f64 * t3278 * t1689 - 0.65854491829355115987e0_f64 * t3287 * t4977 + 0.13170898365871023197e1_f64 * t4981 * t4984 + 0.65854491829355115987e0_f64 * t1087 * t4988 + 0.65854491829355115987e0_f64 * t1087 * t4992 - 0.65854491829355115987e0_f64 * t4996 * t4999 + 0.65854491829355115987e0_f64 * t989 * t1692 - 0.65854491829355115987e0_f64 * t1024 * t5005 + 0.65854491829355115987e0_f64 * t1087 * t5009 + 0.65854491829355115987e0_f64 * t342 * t5012;
    t5015
}
