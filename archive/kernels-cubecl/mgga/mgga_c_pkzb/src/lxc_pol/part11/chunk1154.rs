//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1154/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1154<F: Float>(t1878: F, t218: F, t3761: F, t675: F, t9821: F, t9828: F, t9832: F, t2239: F, t3734: F, t237: F, t9973: F, t3819: F, t6233: F) -> (F, F, F, F, F, F, F) {
    let t27358 = t218 * t1878 * t3761;
    let t27361 = t218 * t675 * t9821;
    let t27370 = t218 * t675 * t9828;
    let t27373 = t218 * t675 * t9832;
    let t27494 = t3734 * t2239;
    let t27501 = t237 * t9973;
    let t27675 = t3819 * t6233;
    (t27358, t27361, t27370, t27373, t27494, t27501, t27675)
}
