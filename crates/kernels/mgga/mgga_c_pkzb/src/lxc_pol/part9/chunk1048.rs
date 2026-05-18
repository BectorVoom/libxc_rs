//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1048/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1048<F: Float>(t16226: F, t534: F, t541: F, t555: F, t137: F, t1835: F, t139: F, t2177: F, t1516: F, t490: F, t4994: F, t1542: F, t1626: F) -> (F, F, F, F, F) {
    let t16230 = F::new(0.5848223622634646207e0) * t555 * t534 * t16226 * t541;
    let t16232 = F::new(1.0) / t137 / t1835;
    let t16250 = F::new(1.0) / t139 / t2177;
    let t16273 = F::new(8.0) * t1516 * t4994 * t490;
    let t16275 = F::new(120.0) * t1542 * t1626;
    (t16230, t16232, t16250, t16273, t16275)
}
