//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 509/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk509<F: Float>(t209: F, t221: F, t5572: F, t1182: F, t605: F, t217: F, t2184: F, t1465: F, t1470: F, t1144: F, t1475: F, t1392: F, t476: F) -> (F, F, F, F, F, F, F) {
    let t5574 = t221 * t5572 * t209;
    let t5577 = t605 * t1182;
    let t5578 = t5577 * t209;
    let t5579 = t221 * t5578;
    let t5582 = t2184 * t217;
    let t5583 = t1465 * t5582;
    let t5585 = F::new(0.25610252642437845428e0) * t5583 * t1470;
    let t5587 = t221 * t1475 * t1144;
    let t5590 = t1392 * t476;
    (t5574, t5578, t5579, t5582, t5585, t5587, t5590)
}
