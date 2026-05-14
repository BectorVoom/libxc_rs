//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1170/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1170<F: Float>(t33717: F, t33726: F, t33728: F, t33731: F, t33734: F, t33719: F, t36671: F, t36672: F, t36673: F, t36674: F, t36675: F, t33741: F, t33743: F, t33746: F, t33750: F, t33753: F) -> (F, F, F, F, F, F) {
    let t36676 = 0.73744819641113281254e-8 * t33717;
    let t36678 = 0.40481770833333333336e-4 * t33726;
    let t36679 = 0.11372686522837130914e-5 * t33728;
    let t36680 = 0.11372686522837130914e-5 * t33731;
    let t36681 = 0.4637672555408563478e-4 * t33734;
    let t36682 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + 0.12650553385416666668e-5 * t33719 + t36678 - t36679 - t36680 + t36681;
    let t36687 = 0.43284943850479925795e-3 * t33741;
    let t36688 = 0.1351988360087076823e-6 * t33743;
    let t36689 = 0.21102562238076876322e-7 * t33746;
    let t36690 = 0.40021712703254065176e-7 * t33750;
    let t36691 = 0.80043425406508130352e-7 * t33753;
    (t36682, t36687, t36688, t36689, t36690, t36691)
}
