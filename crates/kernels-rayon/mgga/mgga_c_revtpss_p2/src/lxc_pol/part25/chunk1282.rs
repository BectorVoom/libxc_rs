//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1282/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1282(t3223: f64, t7131: f64, t1033: f64, t11266: f64, t7120: f64, t11273: f64, t25504: f64, t1047: f64, t11233: f64, t11259: f64, t11281: f64, t11286: f64, t11623: f64, t11776: f64, t12026: f64, t25512: f64, t25522: f64, t3130: f64, t3136: f64, t3157: f64, t7122: f64, t7132: f64, t93750: f64, t93752: f64, t93755: f64, t93758: f64, t93761: f64) -> f64 {
    let t93764 = t3223 * t7131;
    let t93774 = t1033 * t7120 * t11266;
    let t93783 = t11273 * t25504;
    let t93786 = t93750 - 0.17149607247227894789e-2_f64 * t93752 * t11776 - 0.17149607247227894789e-2_f64 * t93755 + 0.42874018118069736972e-3_f64 * t93758 * t11259 + 0.12862205435420921092e-2_f64 * t93761 * t1047 - 0.17149607247227894789e-2_f64 * t93764 * t3130 - 0.85748036236139473944e-3_f64 * t25522 * t12026 - 0.85748036236139473944e-3_f64 * t25522 * t11281 - 0.14291339372689912324e-2_f64 * t25522 * t11286 + 0.43445671692977333464e-1_f64 * t93774 * t1047 - 0.17149607247227894789e-2_f64 * t7132 * t11233 + 0.12862205435420921092e-2_f64 * t25512 * t3136 + 0.42874018118069736972e-3_f64 * t7122 * t11623 + 0.25724410870841842183e-2_f64 * t93783 * t3157;
    t93786
}
