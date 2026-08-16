//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1096/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1096(t70124: f64, t70131: f64, t75928: f64, t75932: f64, t75946: f64, t75951: f64, t75954: f64, t75958: f64, t75964: f64, t75968: f64, t78376: f64, t78377: f64, t78378: f64, t78379: f64, t78384: f64, t78385: f64, t78391: f64) -> f64 {
    let t80386 = t78376 - t78377 - t78378 + t78379 - 0.81756761766873046873e-6_f64 * t70124 - t70131 + 0.58171619854173713844e-5_f64 * t75928 - 0.17451485956252114153e-4_f64 * t75932 + t78384 - t78385 - 0.10511583655740820312e-5_f64 * t75946 - t75951 - 0.93188427318671584242e-2_f64 * t75954 + 0.15531404553111930707e-1_f64 * t75958 + 0.62125618212447722828e-2_f64 * t75964 - t78391 + 0.72714524817717142305e-5_f64 * t75968;
    t80386
}
