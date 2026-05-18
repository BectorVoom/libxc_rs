//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 764/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk764<F: Float>(t7545: F, t7549: F, t7557: F, t7601: F, t7611: F, t7551: F, t7562: F, t7567: F, t7571: F, t7573: F, t7578: F, t7581: F, t7589: F, t7593: F, t7597: F, t7603: F, t7606: F, t7608: F, t7615: F, t7617: F) -> (F, F, F, F, F, F) {
    let t8192 = F::new(0.31448092289604152069e-3) * t7545;
    let t8193 = F::new(0.41930789719472202758e-3) * t7549;
    let t8195 = F::new(0.62896184579208304138e-3) * t7557;
    let t8205 = F::new(0.13073958333333333333e0) * t7601;
    let t8209 = F::new(0.21437009059034868486e-3) * t7611;
    let t8212 = t8192 + t8193 - F::new(0.18868855373762491241e-1) * t7551 - t8195 + F::new(0.68598428988911579156e-2) * t7562 + F::new(0.37737710747524982482e-2) * t7567 + F::new(0.85748036236139473944e-3) * t7571 + F::new(0.25724410870841842184e-2) * t7573 + F::new(0.42874018118069736972e-2) * t7578 - F::new(0.42874018118069736972e-3) * t7581 - F::new(0.28582678745379824648e-3) * t7589 - t7593 / F::new(192.0) - F::new(0.7640625e-2) * t7597 - t8205 + F::new(0.17149607247227894789e-2) * t7603 - F::new(0.34299214494455789578e-2) * t7606 - F::new(0.17149607247227894789e-2) * t7608 + t8209 + F::new(0.32012600194825403606e-1) * t7615 - F::new(0.16006300097412701803e-1) * t7617;
    (t8192, t8193, t8195, t8205, t8209, t8212)
}
