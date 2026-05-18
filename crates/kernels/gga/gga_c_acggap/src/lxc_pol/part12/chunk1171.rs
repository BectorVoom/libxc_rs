//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1171/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1171<F: Float>(t34421: F, t34429: F, t34433: F, t30534: F, t30536: F, t30541: F, t30544: F, t30547: F, t30559: F, t30561: F, t30565: F, t30570: F, t30582: F, t32507: F, t32509: F, t34424: F, t34427: F, t34435: F) -> F {
    let t37087 = F::new(7.0) / F::new(72.0) * t34421;
    let t37090 = F::new(0.21437009059034868486e-2) * t34429;
    let t37093 = F::new(77.0) / F::new(864.0) * t34433;
    let t37095 = F::new(0.38110238327173099532e-2) * t30534 - F::new(0.37737710747524982483e-2) * t30536 + F::new(0.16006300097412701803e-1) * t30541 - F::new(0.25724410870841842184e-1) * t30544 - F::new(0.10289764348336736874e-1) * t30547 + F::new(0.16772315887788881103e-2) * t30559 + F::new(0.41930789719472202758e-2) * t30561 + F::new(0.57165357490759649296e-3) * t30565 - t37087 - t34424 / F::new(16.0) - t34427 / F::new(32.0) - t32507 - t37090 - F::new(0.37737710747524982482e-1) * t30570 + t32509 + F::new(0.25158473831683321655e-2) * t30582 - t37093 + F::new(0.94344276868812456207e-3) * t34435;
    t37095
}
