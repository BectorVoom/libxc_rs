//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 746/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk746<F: Float>(t7677: F, t7696: F, t7717: F, t7725: F, t7737: F, t7739: F, t7742: F, t7747: F, t7775: F, t7781: F, t7787: F, t7800: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8235 = F::new(0.25724410870841842184e-2) * t7677;
    let t8240 = F::new(0.37737710747524982482e-2) * t7696;
    let t8247 = F::new(0.42874018118069736972e-3) * t7717;
    let t8249 = F::new(0.21437009059034868486e-3) * t7725;
    let t8252 = F::new(0.17149607247227894789e-2) * t7737;
    let t8253 = F::new(0.85748036236139473944e-3) * t7739;
    let t8254 = F::new(0.17149607247227894789e-2) * t7742;
    let t8257 = F::new(0.80031500487063509014e-2) * t7747;
    let t8268 = F::new(0.19055119163586549766e-2) * t7775;
    let t8269 = F::new(0.90035438047946447644e-2) * t7781;
    let t8271 = F::new(0.13208198761633743869e-1) * t7787;
    let t8275 = F::new(0.28582678745379824648e-3) * t7800;
    (t8235, t8240, t8247, t8249, t8252, t8253, t8254, t8257, t8268, t8269, t8271, t8275)
}
