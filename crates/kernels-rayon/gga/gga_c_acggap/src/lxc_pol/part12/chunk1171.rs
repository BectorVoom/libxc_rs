//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1171/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1171(t34421: f64, t34429: f64, t34433: f64, t30534: f64, t30536: f64, t30541: f64, t30544: f64, t30547: f64, t30559: f64, t30561: f64, t30565: f64, t30570: f64, t30582: f64, t32507: f64, t32509: f64, t34424: f64, t34427: f64, t34435: f64) -> f64 {
    let t37087 = 7.0_f64 / 72.0_f64 * t34421;
    let t37090 = 0.21437009059034868486e-2_f64 * t34429;
    let t37093 = 77.0_f64 / 864.0_f64 * t34433;
    let t37095 = 0.38110238327173099532e-2_f64 * t30534 - 0.37737710747524982483e-2_f64 * t30536 + 0.16006300097412701803e-1_f64 * t30541 - 0.25724410870841842184e-1_f64 * t30544 - 0.10289764348336736874e-1_f64 * t30547 + 0.16772315887788881103e-2_f64 * t30559 + 0.41930789719472202758e-2_f64 * t30561 + 0.57165357490759649296e-3_f64 * t30565 - t37087 - t34424 / 16.0_f64 - t34427 / 32.0_f64 - t32507 - t37090 - 0.37737710747524982482e-1_f64 * t30570 + t32509 + 0.25158473831683321655e-2_f64 * t30582 - t37093 + 0.94344276868812456207e-3_f64 * t34435;
    t37095
}
