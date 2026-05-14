//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1027/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1027<F: Float>(t18364: F, t6710: F, t9438: F, t7014: F, t9552: F, t20843: F, t2487: F, t3177: F, t587: F, t589: F, t9278: F, t1407: F, t9548: F, t20887: F, t9305: F, t21417: F) -> (F, F, F, F, F, F, F) {
    let t30572 = t6710 * t9438 * t18364;
    let t30574 = t7014 * t9552;
    let t30575 = 0.1022478025437886658e1 * t30574;
    let t30578 = 0.11928910296775344344e1 * t2487 * t20843 * t3177;
    let t30606 = t587 * t589 * t9278;
    let t30607 = 0.1022478025437886658e1 * t30606;
    let t30629 = 0.17041300423964777634e0 * t1407 * t9548;
    let t30631 = 0.29792074959875355558e-1 * t9305 * t20887;
    let t30633 = 0.11916829983950142223e0 * t9305 * t21417;
    (t30572, t30575, t30578, t30607, t30629, t30631, t30633)
}
