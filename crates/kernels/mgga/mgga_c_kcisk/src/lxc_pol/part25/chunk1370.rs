//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1370/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1370<F: Float>(t12261: F, t9994: F, t9725: F, t34474: F, t9736: F, t113003: F, t113009: F, t113022: F, t116639: F, t116656: F, t116659: F, t116662: F, t116666: F, t116669: F, t20: F, t2454: F, t2801: F, t2807: F, t33180: F, t33290: F, t34594: F, t5463: F, t9999: F) -> (F, F) {
    let t118069 = t12261 * t9994;
    let t118070 = t9725 * t118069;
    let t118091 = 0.34722222222222222222e-2 * t34474 * t9736;
    let t118092 = -t113003 - 0.44675925925925925927e-3 * t118070 - 0.46429444444444444443e-2 * t116639 + 0.13888888888888888889e-1 * t2801 * t5463 * t2454 * t20 * t2807 - 0.52083333333333333333e-2 * t33290 * t9999 * t2807 - 0.60312500000000000001e-2 * t34594 * t33180 - 0.13402777777777777778e-2 * t113009 - 0.17361111111111111111e-2 * t113022 - 0.11607361111111111111e-2 * t116656 + 0.77382407407407407407e-3 * t116659 - 0.23214722222222222222e-2 * t116662 - 0.34822083333333333332e-2 * t116666 + 0.23214722222222222222e-2 * t116669 - t118091;
    (t118069, t118092)
}
