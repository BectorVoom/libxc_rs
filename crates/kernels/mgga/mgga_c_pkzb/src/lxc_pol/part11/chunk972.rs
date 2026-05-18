//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 972/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk972<F: Float>(t10589: F, t10590: F, t10595: F, t10601: F, t158: F, t10502: F, t5356: F, t2632: F, t3396: F, t10556: F, t596: F, t1029: F, t1031: F, t160: F, t162: F, t2631: F, t3431: F, t3435: F, t3438: F) -> (F, F, F, F, F) {
    let t10604 = (t10589 + t10590 + t10595 + t10601) * t158;
    let t10612 = t5356 * t10502;
    let t10615 = t2632 * t3396;
    let t10618 = t596 * t10556;
    let t10621 = -F::new(36.0) * t1029 * t3435 + F::new(9.0) * t1029 * t3438 + F::new(9.0) * t1031 * t3431 - t10604 * t162 + F::new(60.0) * t10612 * t160 - F::new(36.0) * t10615 * t2631 + F::new(3.0) * t10618 * t160;
    (t10604, t10612, t10615, t10618, t10621)
}
