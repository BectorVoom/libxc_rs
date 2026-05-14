//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1006/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1006<F: Float>(t25304: F, t555: F, t25898: F, t1444: F, t543: F, t268: F, t4102: F, t4057: F, t676: F, t26028: F, t9807: F, t9812: F, t2482: F, t7262: F, t814: F, t9821: F) -> (F, F, F, F, F, F, F) {
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94396 = t543 * t1444;
    let t94398 = t268 * t4102 * t94396;
    let t94403 = t676 * t4057;
    let t94418 = t26028 * t9807;
    let t94420 = t26028 * t9812;
    let t94423 = t2482 * t7262 * t814;
    let t94424 = t94423 * t9821;
    (t94390, t94391, t94398, t94403, t94418, t94420, t94424)
}
