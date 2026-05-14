//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 742/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk742<F: Float>(t2810: F, t595: F, t637: F, t2813: F, t1732: F, t2758: F, t625: F, t898: F, t1768: F, t1764: F, t2816: F, t1734: F, t5986: F, t2461: F, t759: F, t761: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7808 = t595 * t2810;
    let t7810 = 0.40020429009866666666e-2 * t7808 * t637;
    let t7811 = t595 * t2813;
    let t7813 = 0.40020429009866666666e-2 * t7811 * t637;
    let t7817 = t2758 * t1732;
    let t7824 = t898 * t625;
    let t7825 = t7824 * t1768;
    let t7827 = t7824 * t1764;
    let t7829 = t595 * t2816;
    let t7831 = 0.40020429009866666666e-2 * t7829 * t637;
    let t7832 = t2758 * t1734;
    let t7849 = 80.0 * t5986;
    let t7861 = 0.571528e-1 * t759 * t2461 * t761;
    (t7810, t7813, t7817, t7825, t7827, t7831, t7832, t7849, t7861)
}
