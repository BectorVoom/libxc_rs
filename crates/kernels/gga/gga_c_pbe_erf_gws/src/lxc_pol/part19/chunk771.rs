//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 771/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk771<F: Float>(t1061: F, t1923: F, t256: F, t1918: F, t2654: F, t2785: F, t582: F, t185: F, t2730: F, t2753: F, t1639: F, t649: F, t1642: F, t1730: F, t1: F, t837: F) -> (F, F, F, F, F, F, F) {
    let t7733 = t1061 * t1923;
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7751 = t582 * t2785;
    let t7753 = 8.0 / 45.0 * t185 * t7751;
    let t7757 = 16.0 / 45.0 * t2730 * t2753;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7775 = 16.0 / 45.0 * t1730 * t2753;
    let t7776 = t1 * t837;
    (t7734, t7736, t7753, t7757, t7759, t7775, t7776)
}
