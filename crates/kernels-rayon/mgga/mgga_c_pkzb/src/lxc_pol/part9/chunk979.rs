//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 979/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk979(t2027: f64, t2030: f64, t7665: f64, t302: f64, t1126: f64, t2104: f64, t2899: f64, t2903: f64, t2912: f64, t2922: f64, t2925: f64, t5675: f64, t5681: f64, t5691: f64, t5945: f64, t5984: f64, t7694: f64, t7696: f64, t7703: f64, t7707: f64, t7712: f64, t7715: f64, t7718: f64, t7720: f64, t7725: f64, t7729: f64, t7733: f64, t7736: f64, t7739: f64, t7742: f64) -> (f64, f64, f64, f64) {
    let t7743 = t2030 * t2027;
    let t7744 = t7665 * t7743;
    let t7745 = t302 * t7744;
    let t7749 = 0.19055119163586549765e-3_f64 * t5675 - 0.28582678745379824648e-3_f64 * t5681 + 0.72409452821628889107e-2_f64 * t5945 * t1126 + t7694 + 0.25724410870841842184e-2_f64 * t2104 * t7696 + 0.85748036236139473944e-3_f64 * t2922 * t7703 - 0.45732285992607719436e-2_f64 * t7707 * t2903 + t7712 - t7715 - t7718 - 0.85748036236139473944e-3_f64 * t2104 * t7720 + 0.45732285992607719436e-2_f64 * t5984 * t2912 + 0.22866142996303859718e-2_f64 * t7725 * t2925 + 0.85748036236139473944e-3_f64 * t2899 * t7729 + 0.42874018118069736972e-3_f64 * t2899 * t7733 + 0.12862205435420921092e-2_f64 * t7736 * t7739 - 0.12862205435420921092e-2_f64 * t7742 * t7745 + t5691 / 216.0_f64;
    (t7743, t7744, t7745, t7749)
}
