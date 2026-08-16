//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2459/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2459(t407: f64, t43819: f64, t3256: f64, t3312: f64, t1094: f64, t11274: f64, t3262: f64, t3311: f64, t409: f64, t11285: f64, t3395: f64, t43776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43889 = f64::powf(t407, -0.25e1_f64);
    let t43895 = 0.31310740740740740741e1_f64 * t43819;
    let t43942 = 0.96141975308641975307e-1_f64 * t43819;
    let t43959 = t3256 * t3312;
    let t43964 = t1094 * t11274;
    let t43969 = t409 / t3311 / t3262;
    let t43984 = t11285 * t3395;
    let t44027 = 0.13388493827160493828e1_f64 * t43776;
    (t43889, t43895, t43942, t43959, t43964, t43969, t43984, t44027)
}
