//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1291/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1291(t11670: f64, t3235: f64, t35848: f64, t35851: f64, t35853: f64, t35858: f64, t35861: f64, t35865: f64, t35867: f64, t35869: f64, t35871: f64, t35875: f64, t35878: f64, t35881: f64, t35883: f64) -> f64 {
    let t35885 = t3235 * t11670;
    let t35887 = 0.54715885245250729722e-5_f64 * t35848 - 0.14678726495025884871e-5_f64 * t35851 + 0.23485962392041415794e-5_f64 * t35853 - 0.11742981196020707897e-4_f64 * t35858 + 0.34197428278281706076e-6_f64 * t35861 + 0.99742499144988309388e-7_f64 * t35865 - 0.66360076029596187856e-5_f64 * t35867 + 0.93943849568165663176e-4_f64 * t35869 + 0.93943849568165663176e-4_f64 * t35871 - 0.3077768545045353547e-5_f64 * t35875 + 0.57083936369545107833e-6_f64 * t35878 - 0.33625214088103148142e-7_f64 * t35881 - 0.11742981196020707897e-4_f64 * t35883 - 0.23485962392041415794e-4_f64 * t35885;
    t35887
}
