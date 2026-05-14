//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 771/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk771<F: Float>(t9461: F, t9814: F, t1339: F, t2212: F, t2722: F, t415: F, t2232: F, t9469: F, t2236: F, t468: F, t2718: F, t9426: F, t9444: F, t9446: F, t9460: F, t9792: F, t9796: F, t9801: F, t9805: F, t9809: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9815 = t9461 * t9814;
    let t9816 = t1339 * t9815;
    let t9818 = t2212 * t2722;
    let t9819 = t415 * t9818;
    let t9821 = t9469 * t2232;
    let t9822 = t415 * t9821;
    let t9824 = t468 * t2236;
    let t9825 = t415 * t9824;
    let t9827 = -0.10416666666666666667e-1 * t9792 * t2718 + 0.40208333333333333335e-2 * t9426 * t9796 - 0.10416666666666666667e-1 * t9801 * t2718 - t9444 - 0.34722222222222222223e-2 * t9446 * t9805 + 0.10416666666666666667e-1 * t9446 * t9809 + 0.10416666666666666667e-1 * t9446 * t9796 + t9460 + 0.16581944444444444444e-2 * t9816 + 0.24872916666666666666e-2 * t9819 - 0.24872916666666666666e-2 * t9822 + 0.16581944444444444444e-2 * t9825;
    (t9815, t9816, t9818, t9819, t9821, t9822, t9824, t9825, t9827)
}
