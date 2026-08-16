//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1643;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta450(t24046: f64, t24062: f64, t539: f64, t22645: f64, t225: f64, t7192: f64, t2091: f64, t3887: f64, t3911: f64, t12021: f64, t3888: f64, t7179: f64, t12030: f64, t12033: f64, t12444: f64, t1375: f64, t1386: f64, t2092: f64, t22639: f64, t22650: f64, t3758: f64, t3882: f64, t3889: f64, t3912: f64, t568: f64, t7194: f64, t7199: f64, t7214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24063, t24064, t24071, t24082, t24088, t24092, t24095) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1643(t24046, t24062, t539, t22645, t225, t7192, t2091, t3887, t3911, t12021, t3888, t7179);
        let t24098 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1644(t12030, t12033, t12444, t1375, t1386, t2092, t22639, t22650, t24064, t24071, t24082, t24088, t24092, t24095, t3758, t3882, t3889, t3912, t568, t7194, t7199, t7214);
    (t24063, t24064, t24071, t24082, t24088, t24092, t24095, t24098)
}
