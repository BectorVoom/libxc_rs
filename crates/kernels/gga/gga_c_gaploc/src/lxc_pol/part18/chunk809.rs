//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 809/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk809<F: Float>(t1865: F, t7290: F, t2365: F, t6111: F, t7221: F, t723: F, t1445: F, t1710: F, t2571: F, t2541: F, t769: F, t313: F, t7143: F) -> (F, F, F, F, F, F) {
    let t7718 = t7290 * t1865;
    let t7719 = t2365 * t7718;
    let t7720 = t6111 * t7719;
    let t7722 = t7221 * t723;
    let t7723 = t1445 * t7722;
    let t7726 = t2571 * t1710;
    let t7727 = t1445 * t7726;
    let t7730 = t769 * t2541;
    let t7733 = t313 * t7143;
    (t7720, t7722, t7723, t7727, t7730, t7733)
}
