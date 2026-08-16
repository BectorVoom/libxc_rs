//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1242/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1242(t7664: f64, t7667: f64, t774: f64, t1066: f64, t18210: f64, t1885: f64, t2009: f64, t2104: f64, t2105: f64, t21456: f64, t21729: f64, t21730: f64, t21746: f64, t21749: f64, t21752: f64, t21755: f64, t2739: f64, t2945: f64, t2976: f64, t302: f64, t5633: f64, t5635: f64, t5984: f64, t758: f64, t761: f64, t7648: f64, t7650: f64, t7673: f64, t7678: f64) -> f64 {
    let t21758 = t7664 * t774 * t7667;
    let t21771 = 0.13719685797782315831e-1_f64 * t5984 * t7673 + 0.68598428988911579154e-2_f64 * t5984 * t7678 - 0.21437009059034868486e-3_f64 * t21729 * t302 * t21456 * t21730 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t2739 * t2009 * t761 - 0.15434646522505105311e-1_f64 * t2945 * t758 * t5633 * t2739 * t1885 - 0.17149607247227894789e-2_f64 * t21746 - 0.85748036236139473944e-3_f64 * t21749 - 0.85748036236139473944e-3_f64 * t21752 - 0.42874018118069736972e-3_f64 * t21755 + 0.42874018118069736972e-3_f64 * t21758 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t2976 * t7648 + 0.68598428988911579154e-2_f64 * t5984 * t7650 + 0.25724410870841842184e-1_f64 * t2945 * t758 * t18210 * t1066 * t5635;
    t21771
}
