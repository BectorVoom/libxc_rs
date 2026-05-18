//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1242/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1242<F: Float>(t7664: F, t7667: F, t774: F, t1066: F, t18210: F, t1885: F, t2009: F, t2104: F, t2105: F, t21456: F, t21729: F, t21730: F, t21746: F, t21749: F, t21752: F, t21755: F, t2739: F, t2945: F, t2976: F, t302: F, t5633: F, t5635: F, t5984: F, t758: F, t761: F, t7648: F, t7650: F, t7673: F, t7678: F) -> F {
    let t21758 = t7664 * t774 * t7667;
    let t21771 = F::new(0.13719685797782315831e-1) * t5984 * t7673 + F::new(0.68598428988911579154e-2) * t5984 * t7678 - F::new(0.21437009059034868486e-3) * t21729 * t302 * t21456 * t21730 - F::new(0.12862205435420921092e-2) * t2104 * t2105 * t2739 * t2009 * t761 - F::new(0.15434646522505105311e-1) * t2945 * t758 * t5633 * t2739 * t1885 - F::new(0.17149607247227894789e-2) * t21746 - F::new(0.85748036236139473944e-3) * t21749 - F::new(0.85748036236139473944e-3) * t21752 - F::new(0.42874018118069736972e-3) * t21755 + F::new(0.42874018118069736972e-3) * t21758 - F::new(0.12862205435420921092e-2) * t2104 * t2105 * t2976 * t7648 + F::new(0.68598428988911579154e-2) * t5984 * t7650 + F::new(0.25724410870841842184e-1) * t2945 * t758 * t18210 * t1066 * t5635;
    t21771
}
