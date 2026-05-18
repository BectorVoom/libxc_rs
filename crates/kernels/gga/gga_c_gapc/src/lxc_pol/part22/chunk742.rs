//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 742/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk742<F: Float>(t1577: F, t8399: F, t2936: F, t2937: F, t4644: F, t2952: F, t4865: F, t4868: F, t8362: F, t1005: F, t4883: F, t4885: F) -> (F, F, F, F, F, F) {
    let t8556 = t8399 * t1577;
    let t8557 = t2936 * t8556;
    let t8559 = t2937 * t4644;
    let t8560 = t2936 * t8559;
    let t8562 = t2952 * t4865;
    let t8563 = t8362 * t4868;
    let t8564 = t8562 * t8563;
    let t8566 = t1005 * t4883;
    let t8567 = t2937 * t4885;
    (t8557, t8560, t8562, t8564, t8566, t8567)
}
