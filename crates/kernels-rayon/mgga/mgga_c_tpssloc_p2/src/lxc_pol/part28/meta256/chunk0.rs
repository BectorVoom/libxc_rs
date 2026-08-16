//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1111/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1111(t25: f64, t265: f64, t394: f64, t202: f64, t7109: f64, t1877: f64, t193: f64, t2057: f64, t2522: f64, t7114: f64, t776: f64, t868: f64, t870: f64, t2064: f64, t40: f64, t606: f64, t607: f64, t6542: f64, t6671: f64, t7110: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7125 = t202 * t7109;
    let t7130 = -t1877 * t7114 * t868 + t193 * t7125 * t870 + 3.0_f64 * t2057 * t2522 * t776;
    let t7131 = piecewise3(t395, 0.0_f64, t7130);
    let t7136 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t2057 * t6542 + t1877 * t7110 * t25 / 2.0_f64 - t1877 * t7114 * t6671 / 2.0_f64 + t1877 * t2057 * t606 / 2.0_f64, t2064 * t607 / 2.0_f64 + t7131 * t40 / 2.0_f64);
    (t7125, t7130, t7131, t7136)
}
