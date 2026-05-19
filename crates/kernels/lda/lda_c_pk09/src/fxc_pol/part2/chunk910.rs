//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 910/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk910<F: Float>(t1179: F, t2447: F, t1185: F, t1151: F, t2452: F, t2451: F, t1161: F, t4821: F, t4847: F, t4852: F, t5: F, t2962: F) -> (F, F, F, F, F, F, F) {
    let t9656 = t2447 * t1179;
    let t9657 = t9656 * t1185;
    let t9660 = t1151 * t2452;
    let t9662 = t1179 * t2451;
    let t9663 = t9662 * t1161;
    let t9664 = t4821 * t9663;
    let t9666 = t4847 * t2451;
    let t9669 = t4852 * t2451;
    let t9670 = t9669 * t1185;
    let t9673 = F::cast_from(5.043763671738963_f64) * t5;
    let t9674 = F::cast_from(1.8058298823301853_f64) * t2962;
    (t9657, t9660, t9664, t9666, t9670, t9673, t9674)
}
