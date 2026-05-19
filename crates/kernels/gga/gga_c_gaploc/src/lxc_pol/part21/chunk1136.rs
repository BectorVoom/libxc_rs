//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1136/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1136<F: Float>(t30292: F, t9287: F, t1305: F, t2476: F, t9438: F, t9439: F, t6974: F, t9441: F, t7014: F, t9450: F, t1411: F, t3177: F, t587: F) -> (F, F, F, F, F) {
    let t30294 = F::cast_from(0.29792074959875355558e-1_f64) * t30292 * t9287;
    let t30297 = t2476 * t9438 * t9439 * t1305;
    let t30299 = t6974 * t9441;
    let t30305 = t7014 * t9450;
    let t30323 = F::cast_from(0.11928910296775344344e1_f64) * t587 * t1411 * t3177;
    (t30294, t30297, t30299, t30305, t30323)
}
