//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1143/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1143<F: Float>(t7014: F, t9364: F, t2365: F, t23911: F, t4391: F, t20065: F, t544: F, t9287: F, t1391: F, t587: F, t9547: F, t20117: F, t883: F) -> (F, F, F, F, F, F) {
    let t30779 = t7014 * t9364;
    let t30780 = F::new(0.76685851907841499352e0) * t30779;
    let t30788 = F::new(0.3575048995185042667e0) * t4391 * t2365 * t23911;
    let t30789 = t544 * t20065;
    let t30791 = F::new(0.29792074959875355558e-1) * t30789 * t9287;
    let t30793 = t587 * t1391 * t9547;
    let t30794 = F::new(0.5396411800922179584e0) * t30793;
    let t30802 = t883 * t20117;
    (t30780, t30788, t30789, t30791, t30794, t30802)
}
