//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 479/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk479(t344: f64, t349: f64, t3810: f64, t3814: f64, t3826: f64, t3839: f64, t3851: f64, t4895: f64, t4928: f64, t5163: f64, t5169: f64, t5178: f64, t5181: f64, t5184: f64, t5187: f64, t5194: f64, t5199: f64, t5204: f64, t5207: f64, t793: f64, t797: f64, t838: f64, t854: f64, t861: f64) -> f64 {
    let t5210 = -0.15931384926072697607e-1_f64 * t3826 * t5169 + 0.2230393889650177665e-1_f64 * t3810 * t5163 + 0.47896966807455234256e0_f64 * t3814 * t5163 - 0.23948483403727617128e0_f64 * t3851 * t5169 + 0.53104616420242325356e-2_f64 * t3839 * t5178 + 0.11151969448250888325e-1_f64 * t3810 * t5181 + 0.18586615747084813875e-2_f64 * t861 * t5184 - 0.31862769852145395214e-2_f64 * t854 * t5187 + 0.26552308210121162678e-3_f64 * t344 * t4895 - 0.26552308210121162678e-3_f64 * t349 * t4928 + 0.15965655602485078086e0_f64 * t838 * t5194 + 0.79828278012425390428e-1_f64 * t838 * t5184 - 0.59871208509319042821e-1_f64 * t797 * t5199 - 0.11974241701863808564e0_f64 * t797 * t5187 + 0.79828278012425390428e-1_f64 * t793 * t5204 + 0.39914139006212695214e-1_f64 * t793 * t5207;
    t5210
}
