//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1414/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1414<F: Float>(t745: F, t7803: F, t5435: F, t704: F, t898: F, t5431: F, t963: F, t1745: F, t2747: F, t22387: F, t1422: F, t2755: F, t21837: F, t21843: F, t21846: F, t21858: F, t22382: F, t22386: F, t22390: F, t22395: F) -> (F,) {
    let t26747 = t7803 * t745;
    let t26750 = t898 * t704 * t5435;
    let t26752 = t963 * t5431;
    let t26754 = t2747 * t1745;
    let t26755 = 0.17544670867903938621e1 * t26754;
    let t26756 = 192.0 * t22387;
    let t26758 = 96.0 * t1422 * t2755;
    let t26760 = 0.17544670867903938621e1 * t26747 - 0.3903689268108626343e0 * t26750 + 0.5848223622634646207e0 * t26752 + t26755 + t21837 + t21843 + t22382 + t21846 + t22386 - t26756 + t26758 + 0.31580407562227089518e2 * t22390 - t21858 - t22395;
    (t26760,)
}
