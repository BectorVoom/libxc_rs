//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 743/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk743<F: Float>(t1985: F, t226: F, t1913: F, t20: F, t2004: F, t163: F, t169: F, t684: F, t784: F, t2019: F, t299: F, t4577: F, t148: F, t1602: F, t547: F, t1964: F, t536: F) -> (F, F, F, F, F, F, F) {
    let t5952 = 4.0 * t226 * t1985;
    let t5953 = t1913 * t20;
    let t5954 = t5953 * t2004;
    let t5969 = t169 * t784 * t684 * t163;
    let t5973 = t169 * t299 * t2019 * t163;
    let t5975 = t4577 * t163;
    let t5977 = 0.31505407223141117834e-1 * t148 * t5975;
    let t5980 = t1602 * t547;
    let t5982 = t536 * t1964;
    (t5952, t5954, t5969, t5973, t5977, t5980, t5982)
}
