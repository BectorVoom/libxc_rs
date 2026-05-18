//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1380/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1380<F: Float>(t33875: F, t33878: F, t33881: F, t33885: F, t33888: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36742 = F::new(0.78582449132890172432e-8) * t33875;
    let t36743 = F::new(0.20240885416666666668e-4) * t33878;
    let t36744 = F::new(0.57920616843011475696e-5) * t33881;
    let t36745 = F::new(0.8446756622939173539e-6) * t33885;
    let t36746 = F::new(0.13493923611111111112e-4) * t33888;
    let t36749 = F::new(0.58364997692245511715e-8) * t33897;
    let t36750 = F::new(0.21102562238076876322e-7) * t33899;
    let t36751 = F::new(0.2748593934505475288e-6) * t33902;
    let t36752 = F::new(0.36652500116630512966e-6) * t33904;
    let t36753 = F::new(0.13493923611111111112e-4) * t33908;
    let t36754 = F::new(0.20240885416666666668e-3) * t33911;
    let t36755 = F::new(0.12290803273518880209e-8) * t33914;
    (t36742, t36743, t36744, t36745, t36746, t36749, t36750, t36751, t36752, t36753, t36754, t36755)
}
