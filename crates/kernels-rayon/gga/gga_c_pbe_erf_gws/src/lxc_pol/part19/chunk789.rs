//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 789/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk789(t2004: f64, t5953: f64, t163: f64, t169: f64, t684: f64, t784: f64, t4577: f64, t148: f64, t1964: f64, t536: f64, t147: f64, t413: f64) -> (f64, f64, f64, f64, f64) {
    let t5954 = t5953 * t2004;
    let t5969 = t169 * t784 * t684 * t163;
    let t5975 = t4577 * t163;
    let t5977 = 0.31505407223141117834e-1_f64 * t148 * t5975;
    let t5982 = t536 * t1964;
    let t5984 = t413 * t147;
    (t5954, t5969, t5977, t5982, t5984)
}
