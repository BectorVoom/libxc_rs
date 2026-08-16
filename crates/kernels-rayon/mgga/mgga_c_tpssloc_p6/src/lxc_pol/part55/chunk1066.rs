//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1066/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1066(t25: f64, t265: f64, t394: f64, t30952: f64, t30776: f64, t40: f64, t607: f64, t8678: f64, t191: f64, t192: f64, t7412: f64, t2020: f64, t6997: f64, t8690: f64, t7000: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31823 = piecewise3(t395, 0.0_f64, t30952);
    let t31828 = piecewise3(t115, t30776, t31823 * t40 / 2.0_f64 + t8678 * t607 / 2.0_f64);
    let t31832 = t7412 * t191 * t192;
    let t31833 = t31832 * t2020;
    let t31834 = t8690 * t6997;
    let t31835 = t8690 * t7000;
    (t31823, t31828, t31832, t31833, t31834, t31835)
}
