//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1050/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1050(t25: f64, t265: f64, t394: f64, t1068: f64, t1070: f64, t193: f64, t30920: f64, t30924: f64, t30930: f64, t30952: f64, t336: f64, t4700: f64, t6818: f64, t6822: f64, t30776: f64, t40: f64, t607: f64, t8425: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t30953 = piecewise3(t395, t1070 * t193 * t30920 * t336 - t1068 * t30924 * t4700 + 2.0_f64 * t1068 * t30930 * t4700 - 2.0_f64 * t4700 * t6818 * t6822, t30952);
    let t30958 = piecewise3(t115, t30776, t30953 * t40 / 2.0_f64 + t8425 * t607 / 2.0_f64);
    (t30953, t30958)
}
