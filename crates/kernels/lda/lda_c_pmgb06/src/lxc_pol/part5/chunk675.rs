//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 675/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk675<F: Float>(t12: F, t153: F, t6673: F, t137: F, t132: F, t1: F, t764: F, t2389: F, t337: F, t395: F, t5974: F, t44: F, t131: F, t178: F, t1887: F, t815: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t6674 = t6673 * t153;
    let t6675 = t137 * t6674;
    let t6677 = t132 * t6675 / 30.0;
    let t6678 = t764 * t1;
    let t6681 = t337 * t2389;
    let t6686 = piecewise3(t13, 0.0, 2.0 * t12 * t5974 - 8.0 * t6678 * t395 + 2.0 * t6681);
    let t6687 = t6686 * t44;
    let t6688 = t6687 * t131;
    let t6690 = t6688 * t178 / 30.0;
    let t6692 = t1887 * t815 / 15.0;
    (t6674, t6675, t6677, t6681, t6687, t6688, t6690, t6692)
}
