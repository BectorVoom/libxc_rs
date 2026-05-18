//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 478/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk478<F: Float>(t344: F, t349: F, t3810: F, t3814: F, t3826: F, t3839: F, t3851: F, t4895: F, t4928: F, t5163: F, t5169: F, t5178: F, t5181: F, t5184: F, t5187: F, t5194: F, t5199: F, t5204: F, t5207: F, t793: F, t797: F, t838: F, t854: F, t861: F) -> F {
    let t5210 = -F::new(0.15931384926072697607e-1) * t3826 * t5169 + F::new(0.2230393889650177665e-1) * t3810 * t5163 + F::new(0.47896966807455234256e0) * t3814 * t5163 - F::new(0.23948483403727617128e0) * t3851 * t5169 + F::new(0.53104616420242325356e-2) * t3839 * t5178 + F::new(0.11151969448250888325e-1) * t3810 * t5181 + F::new(0.18586615747084813875e-2) * t861 * t5184 - F::new(0.31862769852145395214e-2) * t854 * t5187 + F::new(0.26552308210121162678e-3) * t344 * t4895 - F::new(0.26552308210121162678e-3) * t349 * t4928 + F::new(0.15965655602485078086e0) * t838 * t5194 + F::new(0.79828278012425390428e-1) * t838 * t5184 - F::new(0.59871208509319042821e-1) * t797 * t5199 - F::new(0.11974241701863808564e0) * t797 * t5187 + F::new(0.79828278012425390428e-1) * t793 * t5204 + F::new(0.39914139006212695214e-1) * t793 * t5207;
    t5210
}
