//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2169/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2169(t39267: f64, t404: f64, t410: f64, t407: f64, t43819: f64, t1098: f64, t11470: f64, t3256: f64, t3312: f64, t1094: f64, t11274: f64, t3262: f64, t3311: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43880 = 1.0_f64 / t410 / t39267 / t404 / 96.0_f64;
    let t43889 = f64::powf(t407, -0.25e1_f64);
    let t43895 = 0.31310740740740740741e1_f64 * t43819;
    let t43942 = 0.96141975308641975307e-1_f64 * t43819;
    let t43954 = t11470 * t1098;
    let t43959 = t3256 * t3312;
    let t43964 = t1094 * t11274;
    let t43969 = t409 / t3311 / t3262;
    (t43880, t43889, t43895, t43942, t43954, t43959, t43964, t43969)
}
