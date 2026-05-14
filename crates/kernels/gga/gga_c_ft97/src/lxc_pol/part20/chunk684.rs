//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 684/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk684<F: Float>(t2724: F, t4125: F, t816: F, t13596: F, t13593: F, t13600: F, t13603: F, t13607: F, t13611: F, t13614: F, t13618: F, t9639: F, t9642: F, t9648: F, t10373: F, t13625: F, t13629: F, t13633: F, t13635: F, t13637: F, t13639: F, t13643: F, t13645: F, t13648: F, t9645: F) -> (F, F, F, F) {
    let t14774 = t2724 * t4125;
    let t14781 = t816 * t4125;
    let t14788 = 0.22226000364197530866e-1 * t13596;
    let t14798 = 0.10001700163888888889e0 * t13593 - t14788 + 0.14817333576131687243e-1 * t13600 + 0.22226000364197530865e-1 * t13603 + 0.51860667516460905352e-1 * t13607 - 0.88904001456790123461e-1 * t13611 - 0.33339000546296296298e-1 * t13614 + 0.13335600218518518519e0 * t13618 - 0.74086667880658436219e-2 * t9639 + 0.55565000910493827163e-2 * t9648 + 0.74086667880658436217e-2 * t9642;
    let t14809 = -0.13335600218518518519e0 * t13625 - 0.11113000182098765433e-1 * t9645 + 0.77791001274691358028e-1 * t13629 - 0.33339000546296296298e-1 * t13633 - 0.29634667152263374486e-1 * t13635 - 0.4445200072839506173e-1 * t13637 - 0.59269334304526748973e-1 * t13639 + 0.29634667152263374487e-1 * t13643 + t10373 + 0.8890400145679012346e-1 * t13645 - 0.37043333940329218109e-2 * t13648;
    (t14774, t14781, t14798, t14809)
}
