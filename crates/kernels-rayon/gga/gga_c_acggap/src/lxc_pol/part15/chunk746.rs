//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 746/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk746(t7677: f64, t7696: f64, t7717: f64, t7725: f64, t7737: f64, t7739: f64, t7742: f64, t7747: f64, t7775: f64, t7781: f64, t7787: f64, t7800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8235 = 0.25724410870841842184e-2_f64 * t7677;
    let t8240 = 0.37737710747524982482e-2_f64 * t7696;
    let t8247 = 0.42874018118069736972e-3_f64 * t7717;
    let t8249 = 0.21437009059034868486e-3_f64 * t7725;
    let t8252 = 0.17149607247227894789e-2_f64 * t7737;
    let t8253 = 0.85748036236139473944e-3_f64 * t7739;
    let t8254 = 0.17149607247227894789e-2_f64 * t7742;
    let t8257 = 0.80031500487063509014e-2_f64 * t7747;
    let t8268 = 0.19055119163586549766e-2_f64 * t7775;
    let t8269 = 0.90035438047946447644e-2_f64 * t7781;
    let t8271 = 0.13208198761633743869e-1_f64 * t7787;
    let t8275 = 0.28582678745379824648e-3_f64 * t7800;
    (t8235, t8240, t8247, t8249, t8252, t8253, t8254, t8257, t8268, t8269, t8271, t8275)
}
