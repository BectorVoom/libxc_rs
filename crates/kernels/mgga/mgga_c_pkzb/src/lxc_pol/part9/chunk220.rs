//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 220/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk220<F: Float>(t650: F, t677: F, t657: F, t668: F, t673: F, t681: F) -> (F, F, F) {
    let t697 = F::new(0.516475e0) * t650;
    let t700 = F::new(0.104195e0) * t677;
    let t702 = F::new(0.3529725e1) * t668 - t697 + F::new(0.1549425e1) * t657 + F::new(0.6311625e0) * t673 - t700 + F::new(0.312585e0) * t681;
    (t697, t700, t702)
}
