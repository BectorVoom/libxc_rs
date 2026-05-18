//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 796/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk796<F: Float>(t233: F, t6298: F, t1709: F, t2811: F, t313: F, t6272: F, t1727: F, t4836: F, t6276: F) -> (F, F, F, F, F, F, F) {
    let t6299 = t233 * t6298;
    let t6300 = t6299 / F::new(8.0);
    let t6301 = t1709 * t1709;
    let t6302 = t6301 * t2811;
    let t6307 = t313 * t6272;
    let t6310 = t4836 * t1727;
    let t6313 = t313 * t6276;
    let t6316 = t1727 * t1727;
    (t6300, t6301, t6302, t6307, t6310, t6313, t6316)
}
