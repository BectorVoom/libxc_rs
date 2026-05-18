//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1196/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1196<F: Float>(t35436: F, t35447: F, t35451: F, t35456: F, t35458: F, t35469: F, t35471: F, t35475: F, t35479: F, t35439: F, t35442: F, t35445: F, t35449: F, t35454: F, t35460: F, t35464: F, t35467: F) -> F {
    let t37551 = F::new(0.16006300097412701803e0) * t35436;
    let t37555 = F::new(0.80031500487063509014e-2) * t35447;
    let t37557 = F::new(0.64025200389650807212e-1) * t35451;
    let t37559 = F::new(0.21437009059034868486e-2) * t35456;
    let t37560 = F::new(0.4528525289702997898e-1) * t35458;
    let t37564 = F::new(0.10289764348336736873e-1) * t35469;
    let t37565 = F::new(0.19055119163586549766e-2) * t35471;
    let t37566 = F::new(0.14291339372689912324e-2) * t35475;
    let t37567 = F::new(0.57165357490759649296e-3) * t35479;
    let t37568 = -t37551 + t35439 / F::new(12.0) + t35442 / F::new(12.0) + F::new(0.305625e-1) * t35445 + t37555 + F::new(0.68598428988911579156e-2) * t35449 - t37557 - F::new(0.21437009059034868486e-3) * t35454 + t37559 + t37560 + F::new(0.27439371595564631662e-1) * t35460 - F::new(0.47172138434406228104e-2) * t35464 + F::new(0.20579528696673473746e-1) * t35467 - t37564 + t37565 + t37566 + t37567;
    t37568
}
