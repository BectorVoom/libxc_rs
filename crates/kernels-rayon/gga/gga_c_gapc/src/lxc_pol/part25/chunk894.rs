//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 894/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk894(t10158: f64, t996: f64, t3218: f64, t1560: f64, t315: f64, t2160: f64, t2165: f64, t3244: f64, t126: f64, t2190: f64, t284: f64, t10137: f64, t10140: f64, t10144: f64, t10148: f64, t10151: f64, t10154: f64, t10156: f64) -> (f64, f64, f64, f64, f64) {
    let t10159 = t996 * t10158;
    let t10160 = t10159 * t3218;
    let t10162 = t1560 * t315;
    let t10163 = t2160 * t10162;
    let t10165 = t2165 * t3244;
    let t10167 = t126 * t2190;
    let t10168 = t284 * t10167;
    let t10170 = -0.56366309740899397906e-3_f64 * t10137 - 0.18788769913633132635e-4_f64 * t10140 - 0.56366309740899397906e-3_f64 * t10144 + 0.56366309740899397906e-3_f64 * t10148 + 0.56366309740899397906e-3_f64 * t10151 + 0.3556532540941297432e-4_f64 * t10154 + 0.3556532540941297432e-4_f64 * t10156 - 0.82073827867876094584e-5_f64 * t10160 - 0.11135477635479903275e-5_f64 * t10163 - 0.82200868372144955279e-5_f64 * t10165 + 0.28183154870449698953e-3_f64 * t10168;
    (t10160, t10163, t10165, t10168, t10170)
}
