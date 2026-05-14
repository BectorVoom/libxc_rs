//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 991/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk991<F: Float>(t17617: F, t1893: F, t5077: F, t1864: F, t2630: F, t1859: F, t5083: F, t15862: F, t6562: F, t6630: F, t15865: F, t6633: F, t493: F, t6503: F, t6751: F, t1981: F, t6406: F, t6747: F) -> (F, F, F, F, F, F, F, F) {
    let t20569 = 4.0 / 15.0 * t5077 * t17617 * t1893;
    let t20572 = 4.0 / 15.0 * t5077 * t2630 * t1864;
    let t20575 = 2.0 / 9.0 * t5083 * t2630 * t1859;
    let t20577 = 4.0 / 15.0 * t15862 * t6562;
    let t20579 = 4.0 / 15.0 * t15862 * t6630;
    let t20581 = 2.0 / 9.0 * t15865 * t6633;
    let t20584 = 2.0 / 3.0 * t493 * t6751 * t6503;
    let t20587 = 8.0 / 15.0 * t1981 * t6747 * t6406;
    (t20569, t20572, t20575, t20577, t20579, t20581, t20584, t20587)
}
