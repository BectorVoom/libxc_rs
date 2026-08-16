//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1196/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1196(t35436: f64, t35447: f64, t35451: f64, t35456: f64, t35458: f64, t35469: f64, t35471: f64, t35475: f64, t35479: f64, t35439: f64, t35442: f64, t35445: f64, t35449: f64, t35454: f64, t35460: f64, t35464: f64, t35467: f64) -> f64 {
    let t37551 = 0.16006300097412701803e0_f64 * t35436;
    let t37555 = 0.80031500487063509014e-2_f64 * t35447;
    let t37557 = 0.64025200389650807212e-1_f64 * t35451;
    let t37559 = 0.21437009059034868486e-2_f64 * t35456;
    let t37560 = 0.4528525289702997898e-1_f64 * t35458;
    let t37564 = 0.10289764348336736873e-1_f64 * t35469;
    let t37565 = 0.19055119163586549766e-2_f64 * t35471;
    let t37566 = 0.14291339372689912324e-2_f64 * t35475;
    let t37567 = 0.57165357490759649296e-3_f64 * t35479;
    let t37568 = -t37551 + t35439 / 12.0_f64 + t35442 / 12.0_f64 + 0.305625e-1_f64 * t35445 + t37555 + 0.68598428988911579156e-2_f64 * t35449 - t37557 - 0.21437009059034868486e-3_f64 * t35454 + t37559 + t37560 + 0.27439371595564631662e-1_f64 * t35460 - 0.47172138434406228104e-2_f64 * t35464 + 0.20579528696673473746e-1_f64 * t35467 - t37564 + t37565 + t37566 + t37567;
    t37568
}
