//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1134/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1134<F: Float>(t17946: F, t21454: F, t287: F, t5726: F, t2104: F, t5974: F, t7719: F, t7649: F, t2922: F, t7654: F, t774: F, t7659: F, t7664: F, t7667: F, t1066: F, t18210: F, t1885: F, t2009: F, t2105: F, t21456: F, t2739: F, t2945: F, t2976: F, t302: F, t5633: F, t5635: F, t5984: F, t758: F, t761: F, t7648: F, t7650: F, t7673: F, t7678: F) -> (F, F) {
    let t21729 = t17946 * t21454;
    let t21730 = t5726 * t287;
    let t21746 = t2104 * t5974 * t7719;
    let t21749 = t2104 * t5974 * t7649;
    let t21752 = t2922 * t774 * t7654;
    let t21755 = t2922 * t774 * t7659;
    let t21758 = t7664 * t774 * t7667;
    let t21771 = 0.13719685797782315831e-1 * t5984 * t7673 + 0.68598428988911579154e-2 * t5984 * t7678 - 0.21437009059034868486e-3 * t21729 * t302 * t21456 * t21730 - 0.12862205435420921092e-2 * t2104 * t2105 * t2739 * t2009 * t761 - 0.15434646522505105311e-1 * t2945 * t758 * t5633 * t2739 * t1885 - 0.17149607247227894789e-2 * t21746 - 0.85748036236139473944e-3 * t21749 - 0.85748036236139473944e-3 * t21752 - 0.42874018118069736972e-3 * t21755 + 0.42874018118069736972e-3 * t21758 - 0.12862205435420921092e-2 * t2104 * t2105 * t2976 * t7648 + 0.68598428988911579154e-2 * t5984 * t7650 + 0.25724410870841842184e-1 * t2945 * t758 * t18210 * t1066 * t5635;
    (t21730, t21771)
}
