//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1099/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1099<F: Float>(t4054: F, t6: F, t4687: F, t5407: F, t505: F, t681: F, t5199: F, t5214: F, t5217: F, t5216: F, t5215: F, t1509: F, t1666: F) -> (F, F, F, F, F, F, F) {
    let t21283 = t4054 * t6;
    let t21369 = t5407 * t4687;
    let t21625 = t681 * t505;
    let t21631 = t5214 * t5199 * t5217;
    let t21642 = t5216 * t6;
    let t21643 = t5215 * t21642;
    let t21649 = t1666 * t1509;
    (t21283, t21369, t21625, t21631, t21642, t21643, t21649)
}
