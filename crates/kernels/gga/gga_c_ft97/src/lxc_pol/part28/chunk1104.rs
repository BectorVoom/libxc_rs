//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1104/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1104<F: Float>(t138725: F, t138761: F, t138770: F, t138784: F, t145168: F, t145419: F, t147231: F, t147234: F, t147238: F, t147243: F, t147248: F, t147253: F, t147258: F, t147262: F, t147266: F, t147271: F, t23701: F, t23825: F, t40087: F, t40227: F, t94401: F, t94429: F, t94514: F, t94524: F, t94530: F, t94535: F) -> F {
    let t147273 = F::new(0.10069900737806194568e-1) * t138725 + F::new(0.40279602951224778273e-1) * t23701 * t145419 - F::new(0.6041940442683716741e-1) * t94524 * t147231 + F::new(0.6041940442683716741e-1) * t94535 * t147234 - F::new(0.10069900737806194568e-1) * t138761 + F::new(0.36251642656102300446e0) * t94514 * t147238 - F::new(0.36251642656102300446e0) * t94401 * t147238 + F::new(0.4445955829703778972e-1) * t147243 * t145168 + F::new(0.6041940442683716741e-1) * t94429 * t147231 + F::new(0.6041940442683716741e-1) * t94429 * t147248 + F::new(0.82108427773942439976e0) * t40087 * t147253 - F::new(0.41054213886971219988e0) * t40227 * t147258 - F::new(0.6041940442683716741e-1) * t138770 - F::new(0.6041940442683716741e-1) * t94530 * t147262 + F::new(0.35314306798406949389e-2) * t138784 + F::new(0.14500657062440920178e1) * t23825 * t147266 - F::new(0.6041940442683716741e-1) * t147271;
    t147273
}
