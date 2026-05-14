//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 760/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk760<F: Float>(t7221: F, t723: F, t1445: F, t1710: F, t2571: F, t2541: F, t769: F, t313: F, t7143: F, t1645: F, t1716: F, t2667: F, t2628: F, t2657: F, t1457: F, t7259: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7722 = t7221 * t723;
    let t7723 = t1445 * t7722;
    let t7726 = t2571 * t1710;
    let t7727 = t1445 * t7726;
    let t7730 = t769 * t2541;
    let t7733 = t313 * t7143;
    let t7736 = t1645 * t1716;
    let t7739 = t2667 * t1710;
    let t7740 = t1445 * t7739;
    let t7743 = t2657 * t2628;
    let t7747 = t1457 * t7259;
    (t7722, t7723, t7727, t7730, t7733, t7736, t7740, t7743, t7747)
}
