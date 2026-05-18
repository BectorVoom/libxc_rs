//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1422/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1422<F: Float>(t11478: F, t9478: F, t11469: F, t1828: F, t3748: F, t11474: F, t1834: F, t313: F, t3951: F, t3957: F, t11376: F, t22723: F, t412: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30760 = t11478 * t9478;
    let t30763 = t11469 * t1828;
    let t30764 = t3748 * t30763;
    let t30767 = t11474 * t9478;
    let t30771 = t3951 * t313 * t1834;
    let t30772 = t3748 * t30771;
    let t30776 = t3957 * t313 * t1834;
    let t30777 = t11376 * t30776;
    let t30781 = t22723 * t412 * t30776;
    (t30760, t30763, t30764, t30767, t30771, t30772, t30776, t30777, t30781)
}
