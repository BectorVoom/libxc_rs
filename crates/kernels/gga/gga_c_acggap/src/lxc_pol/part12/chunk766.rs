//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 766/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk766<F: Float>(t7677: F, t7696: F, t7717: F, t7725: F, t7737: F, t7739: F, t7742: F, t7682: F, t7686: F, t7690: F, t7694: F, t7698: F, t7702: F, t7706: F, t7710: F, t7713: F, t7715: F, t7721: F, t7729: F, t7734: F) -> (F, F, F, F, F, F, F, F) {
    let t8235 = F::new(0.25724410870841842184e-2) * t7677;
    let t8240 = F::new(0.37737710747524982482e-2) * t7696;
    let t8247 = F::new(0.42874018118069736972e-3) * t7717;
    let t8249 = F::new(0.21437009059034868486e-3) * t7725;
    let t8252 = F::new(0.17149607247227894789e-2) * t7737;
    let t8253 = F::new(0.85748036236139473944e-3) * t7739;
    let t8254 = F::new(0.17149607247227894789e-2) * t7742;
    let t8255 = t8235 - F::new(0.37737710747524982483e-2) * t7682 + F::new(0.80031500487063509014e-2) * t7686 + F::new(0.64311027177104605458e-2) * t7690 + F::new(0.12862205435420921092e-2) * t7694 - t8240 - F::new(0.85748036236139473944e-3) * t7698 - F::new(0.85748036236139473944e-3) * t7702 - F::new(0.42874018118069736972e-3) * t7706 + F::new(0.12579236915841660828e-2) * t7710 - F::new(0.17149607247227894789e-2) * t7713 - F::new(0.85748036236139473944e-3) * t7715 - t8247 - F::new(0.42874018118069736972e-3) * t7721 - t8249 - F::new(0.21437009059034868486e-3) * t7729 + F::new(0.17149607247227894789e-2) * t7734 - t8252 - t8253 + t8254;
    (t8235, t8240, t8247, t8249, t8252, t8253, t8254, t8255)
}
