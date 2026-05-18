//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1244/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1244<F: Float>(t20817: F, t236: F, t233: F, t1881: F, t5408: F, t1876: F, t4534: F, t5411: F, t13003: F, t6272: F, t2629: F, t6276: F) -> (F, F, F, F, F, F) {
    let t20818 = t236 * t20817;
    let t20819 = t233 * t20818;
    let t20821 = t1881 * t5408;
    let t20823 = t4534 * t1876;
    let t20824 = t233 * t20823;
    let t20826 = t1881 * t5411;
    let t20828 = t13003 * t6272;
    let t20833 = t2629 * t6276;
    (t20819, t20821, t20824, t20826, t20828, t20833)
}
