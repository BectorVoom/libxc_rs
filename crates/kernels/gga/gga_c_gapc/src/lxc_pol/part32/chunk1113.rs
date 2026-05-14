//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1113/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1113<F: Float>(t17874: F, t35382: F, t35766: F, t10237: F, t3729: F, t11670: F, t3235: F, t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F) -> (F,) {
    let t35881 = t35766 * t35382 * t17874;
    let t35883 = t10237 * t3729;
    let t35885 = t3235 * t11670;
    let t35887 = 0.54715885245250729722e-5 * t35848 - 0.14678726495025884871e-5 * t35851 + 0.23485962392041415794e-5 * t35853 - 0.11742981196020707897e-4 * t35858 + 0.34197428278281706076e-6 * t35861 + 0.99742499144988309388e-7 * t35865 - 0.66360076029596187856e-5 * t35867 + 0.93943849568165663176e-4 * t35869 + 0.93943849568165663176e-4 * t35871 - 0.3077768545045353547e-5 * t35875 + 0.57083936369545107833e-6 * t35878 - 0.33625214088103148142e-7 * t35881 - 0.11742981196020707897e-4 * t35883 - 0.23485962392041415794e-4 * t35885;
    (t35887,)
}
