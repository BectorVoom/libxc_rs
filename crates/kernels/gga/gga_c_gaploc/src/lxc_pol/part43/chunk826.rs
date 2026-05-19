//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 826/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk826<F: Float>(t1441: F, t41596: F, t493: F, t590: F, t1339: F, t1537: F, t34890: F, t6583: F, t9537: F, t10473: F, t2482: F, t9263: F) -> (F, F, F, F) {
    let t41600 = F::cast_from(0.20449560508757733161e1_f64) * t1441 * t493 * t41596 * t590;
    let t41604 = F::cast_from(0.97135412416599232513e1_f64) * t1537 * t1339 * t41596 * t590;
    let t41606 = t6583 * t34890 * t9537;
    let t41607 = F::cast_from(0.19171462976960374838e1_f64) * t41606;
    let t41609 = t9263 * t10473 * t2482;
    (t41600, t41604, t41607, t41609)
}
