//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 945/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk945<F: Float>(t1552: F, t6020: F, t1542: F, t4291: F, t5905: F, t16673: F, t4293: F, t4292: F, t1466: F, t5997: F, t1535: F, t1489: F, t5875: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t17441 = t6020 * t1552;
    let t17443 = t1542 * t4291;
    let t17444 = t17443 * t5905;
    let t17446 = t4293 * t16673;
    let t17447 = t4292 * t17446;
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17451 = t17450 * t1535;
    let t17453 = t5875 * t1489;
    (t17441, t17444, t17446, t17447, t17449, t17451, t17453)
}
