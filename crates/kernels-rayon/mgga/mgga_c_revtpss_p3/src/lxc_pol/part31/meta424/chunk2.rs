//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1521/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1521(t19077: f64, t291: f64, t4719: f64, t4734: f64, t6226: f64, t974: f64, t981: f64, t15170: f64, t15189: f64, t15447: f64, t15457: f64, t15459: f64, t18944: f64, t18961: f64, t18964: f64, t18967: f64, t18970: f64, t18973: f64) -> (f64, f64, f64, f64) {
    let t19079 = 0.621814e-1_f64 * t19077 * t291;
    let t19081 = 0.34631718211362927517e2_f64 * t4719 * t4734;
    let t19082 = t6226 * t974;
    let t19084 = 0.35089341735807877242e1_f64 * t981 * t19082;
    let t19103 = 0.59793333333333333334e0_f64 * t18944 + 0.16431333333333333333e0_f64 * t18961 - 0.54771111111111111112e-1_f64 * t18964 - 0.36514074074074074075e-1_f64 * t18967 - 0.49293999999999999999e0_f64 * t18970 + 0.32862666666666666666e0_f64 * t18973 - t15447 + 0.36514074074074074073e-1_f64 * t15170 - 0.26574814814814814815e0_f64 * t15189 + t15457 + t15459;
    (t19079, t19081, t19084, t19103)
}
