//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1115/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1115<F: Float>(t1962: F, t41154: F, t2411: F, t605: F, t2453: F, t251: F, t25304: F, t7063: F, t860: F, t1113: F, t555: F, t1444: F, t543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92742 = t1962 * t41154;
    let t92790 = t2411 * t605;
    let t93169 = t2453 * t251;
    let t93189 = t25304 * t251;
    let t93341 = t7063 * t860;
    let t94245 = t2411 * t1113;
    let t94382 = t2453 * t555;
    let t94390 = t25304 * t555;
    let t94396 = t543 * t1444;
    (t92742, t92790, t93169, t93189, t93341, t94245, t94382, t94390, t94396)
}
