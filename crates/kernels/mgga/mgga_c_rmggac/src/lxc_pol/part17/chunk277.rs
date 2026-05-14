//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 277/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk277<F: Float>(t1587: F, t1614: F, t1624: F, t1627: F, t1632: F, t1635: F, t305: F, t326: F, t344: F, t349: F, t793: F, t797: F, t838: F, t851: F, t854: F, t861: F) -> (F,) {
    let t1652 = 0.39914139006212695214e-1 * t793 * t1624 - 0.59871208509319042821e-1 * t797 * t1627 + 0.19957069503106347607e-1 * t305 * t1587 - 0.59871208509319042821e-1 * t797 * t1632 + 0.79828278012425390428e-1 * t838 * t1635 - 0.19957069503106347607e-1 * t326 * t1614 + 0.13276154105060581339e-2 * t851 * t1624 - 0.15931384926072697607e-2 * t854 * t1627 + 0.26552308210121162678e-3 * t344 * t1587 - 0.15931384926072697607e-2 * t854 * t1632 + 0.18586615747084813875e-2 * t861 * t1635 - 0.26552308210121162678e-3 * t349 * t1614;
    (t1652,)
}
