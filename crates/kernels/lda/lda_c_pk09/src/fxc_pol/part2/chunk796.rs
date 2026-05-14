//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 796/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk796<F: Float>(t280: F, t6056: F, t9588: F, t1444: F, t309: F, t310: F, t2529: F, t5470: F, t4758: F, t2530: F, t4754: F, t1625: F, t2474: F, t68: F) -> (F, F, F, F, F) {
    let t9589 = t6056 * t280;
    let t9590 = t9588 * t9589;
    let t9592 = t309 * t310 * t1444;
    let t9595 = t5470 * t2529;
    let t9596 = t9595 * t4758;
    let t9599 = t2530 * t4754;
    let t9600 = t9599 * t1625;
    let t9602 = t2474 * t68;
    (t9590, t9592, t9596, t9600, t9602)
}
