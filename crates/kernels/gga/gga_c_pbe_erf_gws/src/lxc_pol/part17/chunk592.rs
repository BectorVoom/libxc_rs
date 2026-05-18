//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 592/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk592<F: Float>(t1697: F, t954: F, t422: F, t1809: F, t639: F, t1640: F, t219: F, t1642: F, t1044: F, t626: F, t1815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2672 = t1697 * t954;
    let t2673 = t2672 * t422;
    let t2674 = t1809 * t2673;
    let t2676 = F::new(8.0) / F::new(45.0) * t639 * t2674;
    let t2677 = t1640 * t219;
    let t2678 = t1642 * t954;
    let t2679 = t2678 * t422;
    let t2680 = t2677 * t2679;
    let t2682 = F::new(4.0) / F::new(27.0) * t639 * t2680;
    let t2683 = t1044 * t626;
    let t2684 = t2683 * t422;
    let t2685 = t1815 * t2684;
    (t2672, t2673, t2674, t2676, t2677, t2678, t2679, t2680, t2682, t2683, t2684, t2685)
}
