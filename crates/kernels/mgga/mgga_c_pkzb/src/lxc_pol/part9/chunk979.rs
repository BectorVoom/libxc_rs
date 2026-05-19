//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 979/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk979<F: Float>(t2027: F, t2030: F, t7665: F, t302: F, t1126: F, t2104: F, t2899: F, t2903: F, t2912: F, t2922: F, t2925: F, t5675: F, t5681: F, t5691: F, t5945: F, t5984: F, t7694: F, t7696: F, t7703: F, t7707: F, t7712: F, t7715: F, t7718: F, t7720: F, t7725: F, t7729: F, t7733: F, t7736: F, t7739: F, t7742: F) -> (F, F, F, F) {
    let t7743 = t2030 * t2027;
    let t7744 = t7665 * t7743;
    let t7745 = t302 * t7744;
    let t7749 = F::cast_from(0.19055119163586549765e-3_f64) * t5675 - F::cast_from(0.28582678745379824648e-3_f64) * t5681 + F::cast_from(0.72409452821628889107e-2_f64) * t5945 * t1126 + t7694 + F::cast_from(0.25724410870841842184e-2_f64) * t2104 * t7696 + F::cast_from(0.85748036236139473944e-3_f64) * t2922 * t7703 - F::cast_from(0.45732285992607719436e-2_f64) * t7707 * t2903 + t7712 - t7715 - t7718 - F::cast_from(0.85748036236139473944e-3_f64) * t2104 * t7720 + F::cast_from(0.45732285992607719436e-2_f64) * t5984 * t2912 + F::cast_from(0.22866142996303859718e-2_f64) * t7725 * t2925 + F::cast_from(0.85748036236139473944e-3_f64) * t2899 * t7729 + F::cast_from(0.42874018118069736972e-3_f64) * t2899 * t7733 + F::cast_from(0.12862205435420921092e-2_f64) * t7736 * t7739 - F::cast_from(0.12862205435420921092e-2_f64) * t7742 * t7745 + t5691 / F::new(216.0);
    (t7743, t7744, t7745, t7749)
}
