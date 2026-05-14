//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1107/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1107<F: Float>(t33653: F, t33660: F, t33671: F, t33674: F, t33680: F, t33682: F, t33687: F, t33690: F, t33692: F, t33694: F, t33697: F, t33701: F, t33704: F, t33707: F, t33710: F, t33714: F, t33717: F, t33719: F, t33726: F, t33728: F, t33731: F, t33734: F) -> (F, F) {
    let t37836 = -0.20596571349374880758e-4 * t33653 + 0.10136107947527008247e-2 * t33660 - 0.10455718062542102956e-9 * t33671 + 0.2023819338830593704e-6 * t33674 - 0.4891547309027777778e-4 * t33680 + 0.4637672555408563478e-4 * t33682 - 0.4891547309027777778e-4 * t33687 + 0.29464735332917200012e-8 * t33690 + 0.11382560960801989336e-6 * t33692 - 0.3601503955298453559e-6 * t33694 + 0.86569887700959851589e-3 * t33697;
    let t37848 = -0.86880925264517213544e-4 * t33701 - 0.86880925264517213544e-4 * t33704 - 0.43440462632258606772e-4 * t33707 - 0.8244751209615223961e-5 * t33710 + 0.14659167650695868203e-4 * t33714 - 0.14748963928222656251e-7 * t33717 + 0.25301106770833333335e-5 * t33719 + 0.8096354166666666667e-4 * t33726 - 0.22745373045674261828e-5 * t33728 - 0.22745373045674261828e-5 * t33731 + 0.9275345110817126956e-4 * t33734;
    (t37836, t37848)
}
