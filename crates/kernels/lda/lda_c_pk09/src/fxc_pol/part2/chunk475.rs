//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 475/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk475<F: Float>(t2558: F, t2610: F, t1409: F, t1411: F, t2502: F, t2505: F, t1389: F, t1391: F, t1393: F, t1395: F, t2542: F, t2546: F) -> (F, F, F) {
    let t2611 = t2558 + t2610;
    let t2615 = t1409 - F::cast_from(0.9421211958699838_f64) * t2502 + t1411 + F::cast_from(0.9421211958699838_f64) * t2505;
    let t2621 = t1389 - F::new(2.0) * t2542 + t1391 + F::new(2.0) * t2546 + t1393 - F::cast_from(0.505765839233979_f64) * t2502 + t1395 + F::cast_from(0.505765839233979_f64) * t2505;
    (t2611, t2615, t2621)
}
