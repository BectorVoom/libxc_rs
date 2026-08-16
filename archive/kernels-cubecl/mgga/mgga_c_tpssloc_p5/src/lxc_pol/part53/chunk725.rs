//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 725/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk725<F: Float>(t225: F, t4266: F, t4143: F, t4145: F, t1509: F, t828: F, t2632: F, t120: F, t4233: F, t1484: F, t852: F, t252: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13042 = t4266 * t225;
    let t13053 = t4143 * t225;
    let t13065 = t4145 * t225;
    let t13223 = t1509 * t828;
    let t13228 = t1509 * t2632;
    let t13242 = t120 * t4233;
    let t13351 = t1484 * t828;
    let t13380 = t852 * t1509;
    let t13384 = t252 * t4233;
    (t13042, t13053, t13065, t13223, t13228, t13242, t13351, t13380, t13384)
}
