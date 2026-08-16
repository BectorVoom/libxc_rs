//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1259/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1259(t297: f64, t56770: f64, t11018: f64, t123: f64, t1325: f64, t1382: f64, t14360: f64, t14390: f64, t16231: f64, t16984: f64, t17045: f64, t2640: f64, t2643: f64, t2668: f64, t2673: f64, t3623: f64, t3821: f64, t40308: f64, t40527: f64, t40539: f64, t49850: f64, t50823: f64, t50828: f64, t50869: f64, t50874: f64, t8114: f64, t8215: f64) -> f64 {
    let t56803 = t56770 * t297;
    let t56831 = -0.40246118008281286364e-2_f64 * t40527 - 0.24147670804968771818e-2_f64 * t40539 + 0.1062950724327133642e5_f64 * t11018 * t14390 * t56803 + 0.73258227843678641352e2_f64 * t50823 - 0.37867004255020313788e0_f64 * t50828 - t50869 / 36.0_f64 - 0.28345352648723563785e5_f64 * t50874 + 0.94667510637550784468e-1_f64 * t2640 * t3821 * t16231 * t1382 * t2643 - 0.2840025319126523534e0_f64 * t2640 * t14360 * t16984 - 0.28345352648723563785e5_f64 * t8114 * t49850 * t8215 * t1325 + 0.47242254414539272975e4_f64 * t11018 * t49850 * t40308 + 0.36629113921839320676e2_f64 * t2668 * t3623 * t2673 * t123 * t17045;
    t56831
}
