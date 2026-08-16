//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 741/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk741<F: Float>(t23024: F, t23189: F, t858: F, t225: F, t2717: F, t2719: F, t6553: F, t1880: F, t1902: F, t2591: F, t252: F, t794: F) -> (F, F, F, F, F, F) {
    let t23190 = t23024 + t23189;
    let t23191 = t858 * t23190;
    let t23195 = t225 * t2717;
    let t23196 = t23195 * t2719;
    let t23197 = t6553 * t23196;
    let t23198 = t1880 * t23197;
    let t23202 = t2591 * t1902;
    let t23204 = t794 * t252;
    (t23190, t23191, t23196, t23198, t23202, t23204)
}
