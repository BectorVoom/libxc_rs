//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2082/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2082(t22715: f64, t268: f64, t405: f64, t1114: f64, t9709: f64, t39267: f64, t404: f64, t410: f64, t407: f64, t1094: f64, t11274: f64, t3262: f64, t3311: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43819 = t268 * t22715 * t405;
    let t43820 = 280.0_f64 / 81.0_f64 * t43819;
    let t43859 = t9709 * t1114;
    let t43880 = 1.0_f64 / t410 / t39267 / t404 / 96.0_f64;
    let t43889 = f64::powf(t407, -0.25e1_f64);
    let t43895 = 0.31310740740740740741e1_f64 * t43819;
    let t43942 = 0.96141975308641975307e-1_f64 * t43819;
    let t43964 = t1094 * t11274;
    let t43969 = t409 / t3311 / t3262;
    (t43819, t43820, t43859, t43880, t43889, t43895, t43942, t43964, t43969)
}
