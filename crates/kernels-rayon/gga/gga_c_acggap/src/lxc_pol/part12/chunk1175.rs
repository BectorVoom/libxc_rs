//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1175/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1175(t34570: f64, t34578: f64, t34590: f64, t34592: f64, t30638: f64, t30640: f64, t30645: f64, t30647: f64, t30649: f64, t30653: f64, t30655: f64, t32540: f64, t34582: f64, t34586: f64, t34595: f64, t34598: f64, t34601: f64, t34604: f64) -> f64 {
    let t37158 = 0.12862205435420921092e-1_f64 * t34570;
    let t37163 = 0.62896184579208304134e-2_f64 * t34578;
    let t37166 = 0.17149607247227894789e-2_f64 * t34590;
    let t37167 = 11.0_f64 / 96.0_f64 * t34592;
    let t37172 = -35.0_f64 / 108.0_f64 * t30638 - 0.21437009059034868486e-3_f64 * t30640 - 0.34299214494455789578e-2_f64 * t30645 + t37158 + 0.25724410870841842184e-2_f64 * t30647 + 0.12862205435420921092e-2_f64 * t30649 - 0.94344276868812456207e-3_f64 * t30653 - 0.85748036236139473944e-3_f64 * t30655 - t32540 + t37163 - 0.25158473831683321655e-2_f64 * t34582 + 0.37737710747524982483e-2_f64 * t34586 - t37166 - t37167 + t34595 / 8.0_f64 + t34598 / 16.0_f64 + t34601 / 32.0_f64 + t34604 / 64.0_f64;
    t37172
}
