//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 766/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk766<F: Float>(t1390: F, t4004: F, t828: F, t531: F, t549: F, t240: F, t72: F, t3829: F, t1386: F, t2482: F, t27: F) -> (F, F, F, F, F, F) {
    let t4006 = t1390 * t828 * t4004;
    let t4010 = F::cast_from(1.0_f64) / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4014 = t4012 * t828 * t3829;
    let t4018 = t2482 * t1386 * t27;
    (t4006, t4010, t4011, t4012, t4014, t4018)
}
