//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 644/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk644<F: Float>(t1646: F, t2484: F, t2372: F, t4663: F, t1648: F, t6771: F, t4638: F, t4676: F, t6756: F, t6761: F, t6766: F, t6769: F, t1815: F, t4664: F, t4667: F, t574: F, t6750: F) -> (F, F, F, F, F, F) {
    let t6774 = t1646 * t2484;
    let t6777 = t4663 * t2372;
    let t6778 = t6777 * t1648;
    let t6780 = t1646 * t6771;
    let t6787 = -0.991e-2 * t6778 + 0.1982e-1 * t6780 + t4676 + 0.13758333333333333333e-2 * t4638 + 0.13758333333333333333e-2 * t6756 - 0.27516666666666666667e-2 * t6761 + 0.8255e-2 * t6766 + 0.8255e-2 * t6769;
    let t6790 = -t4664 * t6750 / 8.0 + t4667 * t2372 / 4.0 + t1815 * t6771 / 4.0 + t6774 * t1648 / 4.0 + t574 * t6787 / 2.0;
    (t6774, t6777, t6778, t6780, t6787, t6790)
}
