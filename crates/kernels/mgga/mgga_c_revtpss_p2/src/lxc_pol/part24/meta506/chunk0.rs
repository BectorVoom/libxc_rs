//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1515/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1515<F: Float>(t22671: F, t706: F, t750: F, t10439: F, t22688: F, t23211: F, t72: F, t757: F, t18263: F, t4305: F, t189: F, t177: F, t762: F) -> (F, F, F, F, F, F) {
    let t76959 = t706 * t750 * t22671;
    let t76965 = t10439 * t750 * t22688;
    let t76972 = t23211 * t72 * t757;
    let t76979 = t18263 * t4305;
    let t77042 = t189 * t22671;
    let t77047 = t23211 * t177 * t762;
    (t76959, t76965, t76972, t76979, t77042, t77047)
}
