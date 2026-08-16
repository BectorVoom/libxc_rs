//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 512/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk512(t349: f64, t3810: f64, t3814: f64, t3819: f64, t3826: f64, t3839: f64, t3851: f64, t6376: f64, t6382: f64, t6387: f64, t6394: f64, t6397: f64, t6400: f64, t6403: f64, t6412: f64, t6415: f64, t6418: f64, t6421: f64, t793: f64, t797: f64, t838: f64, t851: f64, t861: f64) -> f64 {
    let t6424 = -0.15931384926072697607e-1_f64 * t3826 * t6382 - 0.23948483403727617128e0_f64 * t3851 * t6382 + 0.47896966807455234256e0_f64 * t3814 * t6387 - 0.26552308210121162678e-3_f64 * t349 * t6376 + 0.2230393889650177665e-1_f64 * t3810 * t6387 + 0.23948483403727617128e0_f64 * t3814 * t6394 - 0.39914139006212695214e0_f64 * t3839 * t6397 + 0.15965655602485078086e0_f64 * t838 * t6400 - 0.11974241701863808564e0_f64 * t3851 * t6403 + 0.11151969448250888325e-1_f64 * t3810 * t6394 - 0.148692925976678511e-1_f64 * t3819 * t6397 + 0.3717323149416962775e-2_f64 * t861 * t6400 + 0.26552308210121162678e-2_f64 * t851 * t6412 + 0.39914139006212695214e-1_f64 * t793 * t6415 - 0.59871208509319042821e-1_f64 * t797 * t6418 + 0.79828278012425390428e-1_f64 * t838 * t6421;
    t6424
}
