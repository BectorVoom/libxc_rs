//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 827/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk827<F: Float>(t2646: F, t719: F, t256: F, t19: F, t2522: F, t336: F, t714: F, t1061: F, t1923: F, t1918: F, t2654: F, t2785: F, t582: F) -> (F, F, F, F, F) {
    let t7726 = t2646 * t719;
    let t7728 = F::new(2.0) / F::new(3.0) * t7726 * t256;
    let t7729 = t2522 * t19;
    let t7730 = t7729 * t336;
    let t7732 = F::cast_from(0.12155555555555555555e0_f64) * t7730 * t714;
    let t7733 = t1061 * t1923;
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7751 = t582 * t2785;
    (t7728, t7732, t7734, t7736, t7751)
}
