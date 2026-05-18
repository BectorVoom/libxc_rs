//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1308/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1308<F: Float>(t11198: F, t1928: F, t2903: F, t11199: F, t8422: F, t11223: F, t11257: F, t1577: F, t1006: F, t1603: F, t3639: F, t4893: F) -> (F, F, F, F, F) {
    let t35618 = t2903 * t11198 * t1928;
    let t35620 = t8422 * t11199;
    let t35623 = t11257 * t11223 * t1577;
    let t35628 = t1006 * t11223 * t1603;
    let t35631 = t1006 * t3639 * t4893;
    (t35618, t35620, t35623, t35628, t35631)
}
