//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 775/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk775<F: Float>(t7514: F, t7517: F, t7520: F, t7529: F, t7538: F, t7544: F, t7553: F, t7555: F, t7558: F, t7560: F, t7563: F, t7566: F, t7571: F, t7573: F, t7592: F, t7523: F) -> (F, F, F) {
    let t7647 = 0.142419375e1 * t7514 - 0.28483875e1 * t7517 + 0.46074375e0 * t7520 + 0.3071625e0 * t7553 + 0.1898925e1 * t7555 - 0.76790625e-1 * t7558 - 0.32862666666666666666e0 * t7560 + 0.16431333333333333333e0 * t7563 - 0.49293999999999999999e0 * t7566 - 0.59793333333333333333e0 * t7529 + 0.11958666666666666667e1 * t7538 - 0.17938e1 * t7544 - 0.27385555555555555556e0 * t7571 + 0.16431333333333333333e0 * t7573;
    let t7656 = 0.36514074074074074075e0 * t7592;
    let t7657 = 0.93011851851851851854e0 * t7523;
    (t7647, t7656, t7657)
}
