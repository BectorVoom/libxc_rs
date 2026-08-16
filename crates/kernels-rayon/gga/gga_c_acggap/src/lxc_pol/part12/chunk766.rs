//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 766/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk766(t7677: f64, t7696: f64, t7717: f64, t7725: f64, t7737: f64, t7739: f64, t7742: f64, t7682: f64, t7686: f64, t7690: f64, t7694: f64, t7698: f64, t7702: f64, t7706: f64, t7710: f64, t7713: f64, t7715: f64, t7721: f64, t7729: f64, t7734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8235 = 0.25724410870841842184e-2_f64 * t7677;
    let t8240 = 0.37737710747524982482e-2_f64 * t7696;
    let t8247 = 0.42874018118069736972e-3_f64 * t7717;
    let t8249 = 0.21437009059034868486e-3_f64 * t7725;
    let t8252 = 0.17149607247227894789e-2_f64 * t7737;
    let t8253 = 0.85748036236139473944e-3_f64 * t7739;
    let t8254 = 0.17149607247227894789e-2_f64 * t7742;
    let t8255 = t8235 - 0.37737710747524982483e-2_f64 * t7682 + 0.80031500487063509014e-2_f64 * t7686 + 0.64311027177104605458e-2_f64 * t7690 + 0.12862205435420921092e-2_f64 * t7694 - t8240 - 0.85748036236139473944e-3_f64 * t7698 - 0.85748036236139473944e-3_f64 * t7702 - 0.42874018118069736972e-3_f64 * t7706 + 0.12579236915841660828e-2_f64 * t7710 - 0.17149607247227894789e-2_f64 * t7713 - 0.85748036236139473944e-3_f64 * t7715 - t8247 - 0.42874018118069736972e-3_f64 * t7721 - t8249 - 0.21437009059034868486e-3_f64 * t7729 + 0.17149607247227894789e-2_f64 * t7734 - t8252 - t8253 + t8254;
    (t8235, t8240, t8247, t8249, t8252, t8253, t8254, t8255)
}
