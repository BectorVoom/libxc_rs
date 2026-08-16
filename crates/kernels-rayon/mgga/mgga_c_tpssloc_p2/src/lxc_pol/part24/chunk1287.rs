//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1287/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1287(t22724: f64, t22927: f64, t22642: f64, t22643: f64, t6907: f64, t1307: f64, t22633: f64, t22635: f64, t3886: f64, t3888: f64, t12437: f64, t1375: f64, t1378: f64, t1385: f64, t2015: f64, t2016: f64, t22656: f64, t22904: f64, t22913: f64, t3882: f64, t3887: f64, t3912: f64, t39913: f64, t39919: f64, t80744: f64, t81063: f64, t81117: f64, t81183: f64, t81250: f64) -> f64 {
    let t81264 = t22724 * t22927;
    let t81267 = t22642 * t22643 * t6907;
    let t81272 = t22633 * t22635 * t3886 * t3888 * t1307;
    let t81278 = -t80744 - 3.0_f64 * t39913 * t2016 - t1375 * t1378 * (t81063 + t81117 + t81183 + t81250) - 3.0_f64 * t22656 * t3912 - t39919 * t2016 + 2.0_f64 * t1375 * t3887 * t2015 * t12437 + 6.0_f64 * t3882 * t22913 + 0.78134368175290755733e-1_f64 * t81264 + 0.24674011002723396547e-1_f64 * t81267 - 0.9869604401089358619e-1_f64 * t81272 + 6.0_f64 * t1375 * t3887 * t22904 * t1385;
    t81278
}
