//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 746/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk746<F: Float>(t1282: F, t2448: F, t2703: F, t342: F, t38: F, t2209: F, t776: F, t2707: F, t5788: F, t2229: F, t360: F, t5774: F, t5785: F, t5787: F, t5791: F, t5795: F, t5797: F, t63: F, t6968: F, t6971: F, t6975: F, t6978: F, t6980: F, t6984: F, t6987: F, t6989: F) -> (F, F, F, F, F, F) {
    let t6996 = t1282 * t2448;
    let t7002 = F::new(17.53815) * t38 * t2703 * t342;
    let t7005 = F::new(11.6921) * t38 * t776 * t2209;
    let t7008 = F::new(5.84605) * t38 * t2707 * t342;
    let t7009 = F::cast_from(1.2991222222222223_f64) * t5788;
    let t7012 = -F::new(2.93808) * t6968 + F::new(0.73452) * t6971 - t6975 + t6978 - t360 * t6980 / F::new(2.0) - t6984 / F::new(2.0) + t6987 / F::new(6.0) - F::new(29.3808) * t63 * t6989 * t342 + F::new(11.75232) * t63 * t2229 * t2209 + F::new(5.87616) * t63 * t6996 * t342 - t7002 + t7005 + t7008 - t5774 - t5785 - t5787 - t7009 - F::new(4.0) / F::new(9.0) * t5791 + t5795 - F::new(0.97936) * t5797;
    (t6996, t7002, t7005, t7008, t7009, t7012)
}
