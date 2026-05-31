//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 293/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk293<F: Float>(t1263: F, t1272: F, t1268: F, t1275: F, t299: F) -> (F, F, F, F, F, F) {
    let t1295 = F::cast_from(11.879313099038017_f64) * t1263;
    let t1297 = F::cast_from(3.959771033012672_f64) * t1272;
    let t1299 = t1295 - F::cast_from(11.879313099038017_f64) * t1268 + t1297 + F::cast_from(11.879313099038017_f64) * t1275;
    let t1300 = t299 * t299;
    let t1301 = t1300 + F::cast_from(1.0_f64);
    let t1302 = F::cast_from(1.0_f64) / t1301;
    let t1303 = t1299 * t1302;
    (t1295, t1297, t1299, t1301, t1302, t1303)
}
