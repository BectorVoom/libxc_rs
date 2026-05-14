//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1090/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1090<F: Float>(t144: F, t640: F, t2941: F, t3954: F, t3949: F, t8459: F, t3635: F, t8521: F, t11198: F, t1928: F, t2903: F, t11199: F, t8422: F, t11223: F, t11257: F, t1577: F) -> (F, F, F, F, F, F) {
    let t35608 = t640 * t144;
    let t35610 = t2941 * t35608 * t3954;
    let t35613 = t8459 * t35608 * t3949;
    let t35615 = t8521 * t3635;
    let t35618 = t2903 * t11198 * t1928;
    let t35620 = t8422 * t11199;
    let t35623 = t11257 * t11223 * t1577;
    (t35610, t35613, t35615, t35618, t35620, t35623)
}
