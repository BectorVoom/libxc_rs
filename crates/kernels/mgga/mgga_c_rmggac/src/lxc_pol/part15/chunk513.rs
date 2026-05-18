//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 513/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk513<F: Float>(t349: F, t3810: F, t3814: F, t3819: F, t3826: F, t3839: F, t3851: F, t6376: F, t6382: F, t6387: F, t6394: F, t6397: F, t6400: F, t6403: F, t6412: F, t6415: F, t6418: F, t6421: F, t793: F, t797: F, t838: F, t851: F, t861: F) -> F {
    let t6424 = -F::new(0.15931384926072697607e-1) * t3826 * t6382 - F::new(0.23948483403727617128e0) * t3851 * t6382 + F::new(0.47896966807455234256e0) * t3814 * t6387 - F::new(0.26552308210121162678e-3) * t349 * t6376 + F::new(0.2230393889650177665e-1) * t3810 * t6387 + F::new(0.23948483403727617128e0) * t3814 * t6394 - F::new(0.39914139006212695214e0) * t3839 * t6397 + F::new(0.15965655602485078086e0) * t838 * t6400 - F::new(0.11974241701863808564e0) * t3851 * t6403 + F::new(0.11151969448250888325e-1) * t3810 * t6394 - F::new(0.148692925976678511e-1) * t3819 * t6397 + F::new(0.3717323149416962775e-2) * t861 * t6400 + F::new(0.26552308210121162678e-2) * t851 * t6412 + F::new(0.39914139006212695214e-1) * t793 * t6415 - F::new(0.59871208509319042821e-1) * t797 * t6418 + F::new(0.79828278012425390428e-1) * t838 * t6421;
    t6424
}
