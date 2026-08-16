//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1288/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1288<F: Float>(t11670: F, t3235: F, t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F, t35881: F, t35883: F) -> F {
    let t35885 = t3235 * t11670;
    let t35887 = F::cast_from(0.54715885245250729722e-5_f64) * t35848 - F::cast_from(0.14678726495025884871e-5_f64) * t35851 + F::cast_from(0.23485962392041415794e-5_f64) * t35853 - F::cast_from(0.11742981196020707897e-4_f64) * t35858 + F::cast_from(0.34197428278281706076e-6_f64) * t35861 + F::cast_from(0.99742499144988309388e-7_f64) * t35865 - F::cast_from(0.66360076029596187856e-5_f64) * t35867 + F::cast_from(0.93943849568165663176e-4_f64) * t35869 + F::cast_from(0.93943849568165663176e-4_f64) * t35871 - F::cast_from(0.3077768545045353547e-5_f64) * t35875 + F::cast_from(0.57083936369545107833e-6_f64) * t35878 - F::cast_from(0.33625214088103148142e-7_f64) * t35881 - F::cast_from(0.11742981196020707897e-4_f64) * t35883 - F::cast_from(0.23485962392041415794e-4_f64) * t35885;
    t35887
}
