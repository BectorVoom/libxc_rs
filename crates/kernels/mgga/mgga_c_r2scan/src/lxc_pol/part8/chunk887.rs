//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 887/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk887<F: Float>(t1745: F, t963: F, t2747: F, t745: F, t5896: F, t2810: F, t595: F, t637: F, t2813: F, t1732: F, t2758: F, t625: F, t898: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7798 = t963 * t1745;
    let t7802 = 0.11696447245269292414e1 * t2747 * t745;
    let t7807 = 32.0 * t5896;
    let t7808 = t595 * t2810;
    let t7810 = 0.40020429009866666666e-2 * t7808 * t637;
    let t7811 = t595 * t2813;
    let t7813 = 0.40020429009866666666e-2 * t7811 * t637;
    let t7817 = t2758 * t1732;
    let t7824 = t898 * t625;
    (t7798, t7802, t7807, t7808, t7810, t7811, t7813, t7817, t7824)
}
