//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1142/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1142<F: Float>(t5617: F, t984: F, t25545: F, t5495: F, t11982: F, t1564: F, t1642: F, t22500: F, t22875: F, t22886: F, t22917: F, t22935: F, t25584: F, t25601: F, t25615: F, t25616: F, t25617: F, t26119: F, t3266: F, t379: F, t5501: F, t5618: F, t5624: F, t6414: F, t8411: F, t91501: F, t93861: F, t94038: F) -> (F,) {
    let t100089 = t5617 * t984;
    let t100099 = t5495 * t25545 / 9.0;
    let t100120 = -t22935 * t26119 / 9.0 - t5501 * t1564 * t100089 * t379 / 9.0 - t91501 / 9.0 + 2.0 / 9.0 * t5501 * t94038 * t25601 - t100099 + t25584 * t5624 / 3.0 - t6414 * t22500 / 3.0 - 2.0 / 3.0 * t6414 * t22875 + t6414 * t22886 + 2.0 * t5501 * t8411 * t22917 * t3266 - t5501 * t25615 * t25616 * t11982 / 27.0 - t93861 / 9.0 - 2.0 / 27.0 * t5501 * t1642 * t5618 * t25617;
    (t100120,)
}
