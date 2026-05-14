//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1175/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1175<F: Float>(t100634: F, t11604: F, t1742: F, t25802: F, t415: F, t5569: F, t18: F, t2247: F, t423: F, t53: F, t5589: F, t70: F, t100586: F, t22514: F, t2983: F, t8633: F) -> (F, F, F, F) {
    let t100905 = t100634 * t1742 * t11604;
    let t100910 = 0.29693535778629056444e-4 * t5569 * t415 * t25802;
    let t100915 = t5589 * t2247 * t70 * t423 * t18 * t53;
    let t100932 = t22514 * t8633 * t2983 * t100586;
    (t100905, t100910, t100915, t100932)
}
