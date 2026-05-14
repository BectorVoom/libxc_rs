//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1247/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1247<F: Float>(t11234: F, t18649: F, t1271: F, t2712: F, t955: F, t350: F, t365: F, t7018: F, t11230: F, t18704: F, t18706: F, t18707: F, t8358: F, t8376: F, t8382: F, t8386: F, t8388: F, t8390: F) -> (F, F) {
    let t18716 = 70.1526 * t11234 * t18649;
    let t18718 = t1271 * t2712 * t955;
    let t18721 = t365 * t7018 * t350;
    let t18723 = t18704 + 28.0 / 27.0 * t8358 + t18706 - t18707 + 3.91744 * t8376 + 2.0 / 3.0 * t8382 + 1.95872 * t8386 + 3.91744 * t8388 - 0.97936 * t8390 - 117.5232 * t11230 * t18649 - t18716 + 1.95872 * t18718 + 1.46904 * t18721;
    (t18716, t18723)
}
