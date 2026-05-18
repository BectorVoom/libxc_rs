//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1282/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1282<F: Float>(t3223: F, t7131: F, t1033: F, t11266: F, t7120: F, t11273: F, t25504: F, t1047: F, t11233: F, t11259: F, t11281: F, t11286: F, t11623: F, t11776: F, t12026: F, t25512: F, t25522: F, t3130: F, t3136: F, t3157: F, t7122: F, t7132: F, t93750: F, t93752: F, t93755: F, t93758: F, t93761: F) -> F {
    let t93764 = t3223 * t7131;
    let t93774 = t1033 * t7120 * t11266;
    let t93783 = t11273 * t25504;
    let t93786 = t93750 - F::new(0.17149607247227894789e-2) * t93752 * t11776 - F::new(0.17149607247227894789e-2) * t93755 + F::new(0.42874018118069736972e-3) * t93758 * t11259 + F::new(0.12862205435420921092e-2) * t93761 * t1047 - F::new(0.17149607247227894789e-2) * t93764 * t3130 - F::new(0.85748036236139473944e-3) * t25522 * t12026 - F::new(0.85748036236139473944e-3) * t25522 * t11281 - F::new(0.14291339372689912324e-2) * t25522 * t11286 + F::new(0.43445671692977333464e-1) * t93774 * t1047 - F::new(0.17149607247227894789e-2) * t7132 * t11233 + F::new(0.12862205435420921092e-2) * t25512 * t3136 + F::new(0.42874018118069736972e-3) * t7122 * t11623 + F::new(0.25724410870841842183e-2) * t93783 * t3157;
    t93786
}
