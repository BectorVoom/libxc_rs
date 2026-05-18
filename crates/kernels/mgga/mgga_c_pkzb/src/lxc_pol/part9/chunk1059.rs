//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1059/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1059<F: Float>(t1507: F, t16540: F, t4911: F, t555: F, t1517: F, t1527: F, t4999: F, t5002: F, t1625: F, t1661: F, t83: F, t1639: F, t5155: F) -> (F, F, F, F) {
    let t16544 = F::new(0.6233709278045326953e3) * t555 * t4911 * t16540 * t1507;
    let t16548 = F::new(0.3103560775156404018e4) * t4999 * t1517 * t5002 * t1527;
    let t16550 = t83 * t1661 * t1625;
    let t16552 = t5155 * t1639;
    (t16544, t16548, t16550, t16552)
}
