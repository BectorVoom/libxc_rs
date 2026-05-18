//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 535/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk535<F: Float>(t2677: F, t2679: F, t639: F, t1044: F, t626: F, t422: F, t1815: F, t1000: F, t610: F, t1827: F, t587: F, t1684: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2680 = t2677 * t2679;
    let t2682 = F::new(4.0) / F::new(27.0) * t639 * t2680;
    let t2683 = t1044 * t626;
    let t2684 = t2683 * t422;
    let t2685 = t1815 * t2684;
    let t2687 = F::new(4.0) / F::new(45.0) * t639 * t2685;
    let t2688 = t1000 * t610;
    let t2689 = t1827 * t2688;
    let t2691 = F::new(4.0) / F::new(45.0) * t587 * t2689;
    let t2692 = F::new(4.0) / F::new(45.0) * t1684;
    (t2680, t2682, t2684, t2685, t2687, t2688, t2689, t2691, t2692)
}
