//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1356/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1356<F: Float>(t2028: F, t33197: F, t7261: F, t7638: F, t2642: F, t5520: F, t117633: F, t9740: F, t117687: F, t112872: F, t117674: F, t117688: F, t117690: F, t117694: F, t117699: F, t117702: F, t117705: F, t117715: F, t33196: F, t34400: F, t34419: F, t34422: F, t74798: F, t9743: F) -> (F, F) {
    let t117718 = t7261 * t33197 * t7638 * t2028;
    let t117725 = t7261 * t33197 * t2642 * t5520;
    let t117729 = 0.34722222222222222222e-2 * t9740 * t117633;
    let t117730 = t9740 * t117687;
    let t117732 = -0.77602083333333333334e-3 * t117688 - 0.34722222222222222222e-2 * t117690 * t9743 - 0.34722222222222222222e-2 * t117694 * t9743 - 0.116403125e-2 * t34419 * t117699 + 0.92592592592592592594e-2 * t117702 * t9743 + 0.898632125e-3 * t117705 * t117674 - 0.40208333333333333334e-2 * t112872 * t34400 - 0.10416666666666666667e-1 * t9740 * t7261 * t34422 * t74798 - t117715 - 0.10416666666666666667e-1 * t9740 * t117718 - 0.40208333333333333334e-2 * t33196 * t117718 - 0.52083333333333333333e-2 * t9740 * t117725 - t117729 - 0.69444444444444444445e-2 * t117730;
    (t117725, t117732)
}
