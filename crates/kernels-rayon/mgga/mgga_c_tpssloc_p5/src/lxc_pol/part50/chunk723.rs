//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 723/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk723(t25: f64, t1965: f64, t40: f64, t607: f64, t6678: f64, t6835: f64, t28: f64, t776: f64, t868: f64, t1081: f64, t1877: f64, t1915: f64, t2522: f64, t6666: f64, t6670: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t6840 = piecewise3(t115, t6678, t1965 * t607 / 2.0_f64 + t6835 * t40 / 2.0_f64);
    let t6841 = t28 * t776;
    let t6848 = t28 * t868;
    let t6855 = 3.0_f64 / 2.0_f64 * t2522 * t1915 * t6841 + t1877 * t6666 * t28 / 2.0_f64 - t1877 * t6670 * t6848 / 2.0_f64 + t1877 * t1915 * t1081 / 2.0_f64;
    (t6840, t6841, t6848, t6855)
}
