//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1260/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1260<F: Float>(t1882: F, t26454: F, t26416: F, t100248: F, t101908: F, t103: F, t10969: F, t11618: F, t1339: F, t1825: F, t1901: F, t23353: F, t23374: F, t26145: F, t28: F, t3238: F, t446: F, t452: F, t47768: F, t6538: F, t82: F, t8360: F, t8411: F, t89: F, t93630: F, t93632: F, t93634: F, t93647: F, t93649: F, t93656: F) -> (F,) {
    let t103783 = 2.0 / 9.0 * t1882 * t26454;
    let t103796 = 2.0 / 9.0 * t1882 * t26416;
    let t103816 = 2.0 / 3.0 * t446 * t452 * t3238 * t23353 + t103783 + t89 * t28 * t82 * t101908 * t103 / 3.0 - 8.0 / 27.0 * t93630 + 2.0 / 9.0 * t93632 + t93634 / 9.0 + 8.0 / 81.0 * t93647 + 4.0 / 9.0 * t93649 - 2.0 / 9.0 * t93656 - t103796 - 2.0 / 3.0 * t446 * t452 * t10969 * t23374 + t446 * t452 * t8360 * t6538 / 3.0 + 2.0 / 3.0 * t446 * t452 * t1825 * t26145 - 2.0 * t446 * t8411 * t1339 * t11618 + 10.0 / 81.0 * t1901 * t47768 * t100248;
    (t103816,)
}
