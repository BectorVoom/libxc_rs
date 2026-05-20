//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1646/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1646<F: Float>(t2382: F, t4186: F, t2615: F, t4311: F, t1469: F, t2609: F, t706: F, t80: F, t83: F, t1568: F, t785: F, t780: F) -> (F, F, F, F, F, F, F, F) {
    let t14416 = t2382 * t4186;
    let t14433 = F::new(8.0) * t4311 * t2615;
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14447 = t80 * t4186;
    let t14458 = t83 * t4186;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    (t14416, t14433, t14440, t14441, t14447, t14458, t14472, t14473)
}
