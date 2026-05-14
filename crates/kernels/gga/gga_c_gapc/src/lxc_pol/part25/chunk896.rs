//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 896/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk896<F: Float>(t1062: F, t11670: F, t268: F, t3643: F, t128: F, t6939: F, t10357: F, t2207: F, t10350: F, t3737: F, t6940: F, t2415: F, t959: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11671 = t1062 * t11670;
    let t11673 = t3643 * t268;
    let t11674 = t6939 * t128;
    let t11675 = t11673 * t11674;
    let t11676 = t11675 * t10357;
    let t11678 = t2207 * t128;
    let t11679 = t11673 * t11678;
    let t11680 = t11679 * t10350;
    let t11682 = t3737 * t6940;
    let t11683 = t2415 * t959;
    (t11671, t11673, t11674, t11675, t11676, t11678, t11679, t11680, t11682, t11683)
}
