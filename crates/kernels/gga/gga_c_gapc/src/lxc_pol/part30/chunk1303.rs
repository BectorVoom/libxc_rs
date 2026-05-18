//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1303/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1303<F: Float>(t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F, t33645: F, t33648: F, t33653: F, t33660: F, t33671: F, t33674: F, t33680: F, t33682: F, t33687: F, t33690: F, t33692: F, t33694: F, t33697: F) -> (F, F) {
    let t37823 = -F::new(0.13937148427849636339e-3) * t33614 + F::new(0.19336232562226912507e-7) * t33617 - F::new(0.2051637995368585198e-8) * t33621 - F::new(0.40441273275208837532e-5) * t33625 - F::new(0.40441273275208837532e-5) * t33628 - F::new(0.9275345110817126956e-4) * t33631 + F::new(0.69504740211613770836e-3) * t33634 + F::new(0.13900948042322754167e-2) * t33637 - F::new(0.9275345110817126956e-4) * t33641 - F::new(0.43284943850479925794e-3) * t33645 + F::new(0.13900948042322754167e-2) * t33648;
    let t37836 = -F::new(0.20596571349374880758e-4) * t33653 + F::new(0.10136107947527008247e-2) * t33660 - F::new(0.10455718062542102956e-9) * t33671 + F::new(0.2023819338830593704e-6) * t33674 - F::new(0.4891547309027777778e-4) * t33680 + F::new(0.4637672555408563478e-4) * t33682 - F::new(0.4891547309027777778e-4) * t33687 + F::new(0.29464735332917200012e-8) * t33690 + F::new(0.11382560960801989336e-6) * t33692 - F::new(0.3601503955298453559e-6) * t33694 + F::new(0.86569887700959851589e-3) * t33697;
    (t37823, t37836)
}
