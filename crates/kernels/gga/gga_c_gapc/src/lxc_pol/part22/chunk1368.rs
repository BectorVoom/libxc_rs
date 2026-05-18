//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1368/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1368<F: Float>(t33248: F, t33252: F, t33254: F, t33259: F, t33263: F, t33265: F, t33270: F, t33275: F, t33278: F, t33261: F, t36520: F, t33284: F) -> (F, F) {
    let t36521 = F::new(0.63350674672043801542e-5) * t33248;
    let t36522 = F::new(0.63350674672043801542e-5) * t33252;
    let t36523 = F::new(0.69504740211613770836e-3) * t33254;
    let t36524 = F::new(0.17207124582926432293e-7) * t33259;
    let t36526 = F::new(0.21720231316129303386e-4) * t33263;
    let t36527 = F::new(0.43440462632258606772e-4) * t33265;
    let t36528 = F::new(0.88465285289519332099e-6) * t33270;
    let t36529 = F::new(0.10117831965157323855e-7) * t33275;
    let t36530 = F::new(0.25002399603899953676e-2) * t33278;
    let t36531 = -t36520 + t36521 + t36522 - t36523 - t36524 + F::new(0.97817934710145362362e-6) * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    let t36534 = F::new(0.75031332402051115813e-8) * t33284;
    (t36531, t36534)
}
