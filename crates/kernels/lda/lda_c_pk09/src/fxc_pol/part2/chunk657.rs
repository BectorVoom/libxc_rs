//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 657/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk657<F: Float>(t1519: F, t304: F, t5308: F, t327: F, t5009: F, t309: F, t310: F, t4993: F, t1240: F, t1434: F, t1637: F, t318: F) -> (F, F, F, F, F, F, F) {
    let t5829 = t304 * t1519;
    let t5830 = t5829 * t5308;
    let t5832 = t327 * t5009;
    let t5834 = t309 * t310 * t4993;
    let t5836 = t5832 * t5834 / F::cast_from(3.0_f64);
    let t5838 = t309 * t1434 * t1240;
    let t5840 = t1637 * t5838 / F::cast_from(9.0_f64);
    let t5845 = t318 * t5009;
    (t5829, t5830, t5834, t5836, t5838, t5840, t5845)
}
