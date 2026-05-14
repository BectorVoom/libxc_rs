//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 661/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk661<F: Float>(t1908: F, t7444: F, t4638: F, t5360: F, t6756: F, t6761: F, t6766: F, t6769: F, t1965: F, t2597: F, t1973: F, t2605: F, t4724: F, t5380: F, t5387: F, t6778: F, t6780: F, t6818: F, t6820: F, t6823: F, t6826: F, t6829: F, t6832: F) -> (F, F, F, F, F) {
    let t7445 = t1908 * t7444;
    let t7464 = t5360 + 0.57077777777777777777e-2 * t4638 + 0.57077777777777777777e-2 * t6756 - 0.11415555555555555555e-1 * t6761 + 0.34246666666666666666e-1 * t6766 + 0.34246666666666666666e-1 * t6769;
    let t7467 = t2597 * t1965;
    let t7472 = t2605 * t1973;
    let t7489 = -0.17648625e1 * t6778 + 0.3529725e1 * t6780 + t5380 + 0.17215833333333333333e0 * t4638 + 0.17215833333333333333e0 * t6756 - 0.34431666666666666667e0 * t6761 + 0.103295e1 * t6766 + 0.103295e1 * t6769 + 0.31558125e0 * t6818 + 0.6311625e0 * t6820 + t5387 + 0.69463333333333333333e-1 * t4724 + 0.69463333333333333333e-1 * t6823 - 0.34731666666666666667e-1 * t6826 + 0.20839e0 * t6829 + 0.20839e0 * t6832;
    (t7445, t7464, t7467, t7472, t7489)
}
