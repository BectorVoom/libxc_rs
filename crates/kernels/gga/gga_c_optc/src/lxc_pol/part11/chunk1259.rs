//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1259/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1259<F: Float>(t297: F, t56770: F, t11018: F, t123: F, t1325: F, t1382: F, t14360: F, t14390: F, t16231: F, t16984: F, t17045: F, t2640: F, t2643: F, t2668: F, t2673: F, t3623: F, t3821: F, t40308: F, t40527: F, t40539: F, t49850: F, t50823: F, t50828: F, t50869: F, t50874: F, t8114: F, t8215: F) -> F {
    let t56803 = t56770 * t297;
    let t56831 = -F::cast_from(0.40246118008281286364e-2_f64) * t40527 - F::cast_from(0.24147670804968771818e-2_f64) * t40539 + F::cast_from(0.1062950724327133642e5_f64) * t11018 * t14390 * t56803 + F::cast_from(0.73258227843678641352e2_f64) * t50823 - F::cast_from(0.37867004255020313788e0_f64) * t50828 - t50869 / F::new(36.0) - F::cast_from(0.28345352648723563785e5_f64) * t50874 + F::cast_from(0.94667510637550784468e-1_f64) * t2640 * t3821 * t16231 * t1382 * t2643 - F::cast_from(0.2840025319126523534e0_f64) * t2640 * t14360 * t16984 - F::cast_from(0.28345352648723563785e5_f64) * t8114 * t49850 * t8215 * t1325 + F::cast_from(0.47242254414539272975e4_f64) * t11018 * t49850 * t40308 + F::cast_from(0.36629113921839320676e2_f64) * t2668 * t3623 * t2673 * t123 * t17045;
    t56831
}
