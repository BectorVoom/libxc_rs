//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1227/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1227<F: Float>(t11594: F, t20604: F, t11597: F, t3001: F, t9071: F, t11598: F, t9080: F, t8848: F, t19624: F, t33148: F, t5395: F, t1030: F, t33895: F, t9249: F) -> (F, F, F, F, F, F) {
    let t34550 = t11594 * t20604;
    let t34553 = t9071 * t11597 * t3001;
    let t34555 = t11598 * t9080;
    let t34557 = t11598 * t8848;
    let t34560 = t5395 * t33148 * t19624;
    let t34563 = t1030 * t33895 * t9249;
    (t34550, t34553, t34555, t34557, t34560, t34563)
}
