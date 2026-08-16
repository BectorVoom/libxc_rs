//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2215/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2215(t100007: f64, t16094: f64, t12167: f64, t99984: f64, t12078: f64, t25516: f64, t4954: f64, t15752: f64, t27498: f64, t15596: f64, t15601: f64, t15615: f64, t15910: f64, t15965: f64, t16084: f64, t16128: f64, t16201: f64, t25517: f64, t3097: f64, t4788: f64, t4907: f64, t7132: f64, t93670: f64, t93821: f64) -> (f64, f64) {
    let t100135 = t16094 * t100007;
    let t100138 = t12167 * t99984;
    let t100141 = t12078 * t99984;
    let t100146 = t4954 * t25516;
    let t100160 = 0.57165357490759649296e-3_f64 * t27498 * t15752;
    let t100163 = -0.95275595817932748826e-3_f64 * t100135 * t16128 + 0.25724410870841842183e-2_f64 * t100138 * t16084 - 0.25724410870841842183e-2_f64 * t100141 * t15910 - 0.57165357490759649296e-3_f64 * t25517 * t15965 + 0.57165357490759649296e-3_f64 * t100146 * t3097 + 0.57165357490759649296e-3_f64 * t25517 * t15615 + 0.57165357490759649296e-3_f64 * t93821 * t4788 + 0.47637797908966374413e-3_f64 * t25517 * t15596 + 0.28582678745379824648e-3_f64 * t25517 * t15601 - 0.28582678745379824648e-2_f64 * t7132 * t16201 - t100160 + 0.45732285992607719436e-2_f64 * t93670 * t4907;
    (t100135, t100163)
}
