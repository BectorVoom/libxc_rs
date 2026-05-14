//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 817/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk817<F: Float>(t1648: F, t2556: F, t2562: F, t2567: F, t1044: F, t1697: F, t1413: F, t1815: F, t639: F, t5414: F, t2785: F, t582: F, t185: F, t1033: F, t1795: F, t2730: F, t2753: F) -> (F, F, F, F, F, F, F, F) {
    let t7740 = 16.0 / 45.0 * t1648 * t2556;
    let t7742 = 8.0 / 27.0 * t1648 * t2562;
    let t7744 = 8.0 / 45.0 * t1648 * t2567;
    let t7745 = t1044 * t1697;
    let t7746 = t7745 * t1413;
    let t7747 = t1815 * t7746;
    let t7749 = 8.0 / 45.0 * t639 * t7747;
    let t7750 = 8.0 / 135.0 * t5414;
    let t7751 = t582 * t2785;
    let t7753 = 8.0 / 45.0 * t185 * t7751;
    let t7755 = 4.0 / 15.0 * t1033 * t1795;
    let t7757 = 16.0 / 45.0 * t2730 * t2753;
    (t7740, t7742, t7744, t7749, t7750, t7753, t7755, t7757)
}
