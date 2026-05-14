//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1226/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1226<F: Float>(t1307: F, t3289: F, t1286: F, t1637: F, t6422: F, t108: F, t11392: F, t11594: F, t1564: F, t1647: F, t22863: F, t22907: F, t22908: F, t22924: F, t22928: F, t22932: F, t22935: F, t25558: F, t25564: F, t25577: F, t26117: F, t26125: F, t26130: F, t28: F, t379: F, t5495: F, t5501: F, t5507: F, t6421: F, t984: F) -> (F,) {
    let t102291 = t1307 * t3289;
    let t102312 = t1286 * t1637 * t6422;
    let t102328 = 4.0 / 9.0 * t25577 * t22907 * t22908 * t11594 - t5501 * t1564 * t102291 * t379 / 9.0 + 2.0 * t22935 * t25564 + t1286 * t28 * t22863 * t984 / 6.0 + t5501 * t1564 * t26117 * t1647 / 9.0 - t1286 * t28 * t5507 * t108 * t11392 / 3.0 - 4.0 / 27.0 * t102312 - t25558 * t22924 / 9.0 - t25558 * t22928 / 18.0 - t25558 * t22932 / 27.0 - 2.0 / 3.0 * t5495 * t26125 - 2.0 / 3.0 * t5495 * t26130 - 2.0 / 9.0 * t5501 * t22907 * t6421 * t1647;
    (t102328,)
}
