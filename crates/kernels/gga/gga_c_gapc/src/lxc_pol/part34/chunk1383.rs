//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1383/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1383<F: Float>(t33865: F, t33870: F, t33872: F, t33875: F, t33878: F, t33881: F, t33885: F, t33888: F, t33897: F, t33899: F, t33902: F, t33904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36738 = F::new(0.3077456993052877797e-8) * t33865;
    let t36740 = F::new(0.19336232562226912508e-7) * t33870;
    let t36741 = F::new(0.42205124476153752644e-7) * t33872;
    let t36742 = F::new(0.78582449132890172432e-8) * t33875;
    let t36743 = F::new(0.20240885416666666668e-4) * t33878;
    let t36744 = F::new(0.57920616843011475696e-5) * t33881;
    let t36745 = F::new(0.8446756622939173539e-6) * t33885;
    let t36746 = F::new(0.13493923611111111112e-4) * t33888;
    let t36749 = F::new(0.58364997692245511715e-8) * t33897;
    let t36750 = F::new(0.21102562238076876322e-7) * t33899;
    let t36751 = F::new(0.2748593934505475288e-6) * t33902;
    let t36752 = F::new(0.36652500116630512966e-6) * t33904;
    (t36738, t36740, t36741, t36742, t36743, t36744, t36745, t36746, t36749, t36750, t36751, t36752)
}
