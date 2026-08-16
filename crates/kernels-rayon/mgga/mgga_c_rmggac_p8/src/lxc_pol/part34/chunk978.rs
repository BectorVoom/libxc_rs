//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 978/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk978(t77217: f64, t16503: f64, t35039: f64, t699: f64, t9169: f64, t30221: f64, t3194: f64, t74803: f64, t14703: f64, t289: f64, t623: f64, t71151: f64, t71162: f64, t74779: f64, t74782: f64, t74786: f64, t74800: f64, t77206: f64, t77208: f64, t77209: f64, t77212: f64, t77213: f64, t77214: f64) -> f64 {
    let t77218 = 0.53205749866622299248e-5_f64 * t77217;
    let t77221 = t16503 * t35039 * t699 * t9169;
    let t77222 = 0.42564599893297839398e-5_f64 * t77221;
    let t77224 = 0.39914139006212695214e-1_f64 * t30221 * t3194;
    let t77225 = 0.2727466165424534173e-1_f64 * t74803;
    let t77226 = 0.6505345598561924296e-5_f64 * t74779 - t74782 - 0.19957069503106347607e-1_f64 * t623 * t14703 + t71151 - t77206 + 0.72714524817717142308e-5_f64 * t74786 - t77208 - 0.2363e1_f64 * t289 * t77209 - t77212 - t77213 + t71162 + t77214 - t77218 - t77222 + t74800 + t77224 + t77225;
    t77226
}
