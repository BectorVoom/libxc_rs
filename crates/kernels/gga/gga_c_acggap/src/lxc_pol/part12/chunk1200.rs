//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1200/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1200<F: Float>(t35587: F, t35594: F, t35596: F, t35608: F, t35610: F, t35623: F, t31421: F, t31426: F, t31429: F, t35580: F, t35585: F, t35591: F, t35599: F, t35601: F, t35614: F, t35616: F, t35621: F) -> F {
    let t37622 = F::new(0.85748036236139473944e-3) * t35587;
    let t37624 = F::new(0.85748036236139473944e-3) * t35594;
    let t37625 = F::new(0.25724410870841842184e-2) * t35596;
    let t37631 = F::new(0.41930789719472202758e-3) * t35608;
    let t37632 = F::new(0.11321313224257494745e-1) * t35610;
    let t37636 = F::new(0.12579236915841660828e-2) * t35623;
    let t37637 = F::new(0.25158473831683321655e-2) * t35580 - F::new(0.5031694766336664331e-2) * t35585 + t37622 - F::new(0.64311027177104605458e-2) * t35591 + t37624 + t37625 + F::new(0.12862205435420921092e-1) * t35599 + F::new(0.11321313224257494744e0) * t35601 + F::new(0.4584375e-1) * t31421 - F::new(0.16809375e0) * t31426 - F::new(11.0) / F::new(48.0) * t31429 + t37631 + t37632 - F::new(0.94344276868812456207e-3) * t35614 - F::new(0.31448092289604152068e-2) * t35616 - F::new(0.15724046144802076034e-2) * t35621 + t37636;
    t37637
}
