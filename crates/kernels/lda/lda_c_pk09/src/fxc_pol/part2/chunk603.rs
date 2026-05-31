//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 603/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk603<F: Float>(t1174: F, t253: F, t1185: F, t1197: F, t212: F, t1161: F, t1207: F, t416: F, t1471: F, t747: F, t1525: F, t1435: F, t1546: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4882 = t253 * t1174;
    let t4886 = t1197 * t1185;
    let t4895 = F::cast_from(1.0_f64) / t212;
    let t4910 = t1207 * t1161;
    let t4917 = t1207 * t1185;
    let t4926 = F::cast_from(1.0_f64) / t416;
    let t4943 = t747 * t1471;
    let t4944 = t1525 * t4943;
    let t4945 = F::cast_from(7.200326855928252_f64) * t4944;
    let t4950 = t1546 * t1435;
    (t4882, t4886, t4895, t4910, t4917, t4926, t4943, t4944, t4945, t4950)
}
