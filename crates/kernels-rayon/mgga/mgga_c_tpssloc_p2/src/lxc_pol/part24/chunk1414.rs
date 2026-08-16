//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1414/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1414(t28: f64, t265: f64, t504: f64, t83607: f64, t83654: f64, t83543: f64, t1972: f64, t2250: f64, t23821: f64, t52: f64, t607: f64, t6856: f64, t9258: f64, t22561: f64, t2314: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t83655 = t83607 + t83654;
    let t83656 = piecewise3(t505, 0.0_f64, t83543);
    let t83666 = piecewise3(t401, t83655, t83656 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t23821 * t607 - 3.0_f64 / 2.0_f64 * t6856 * t2250 - t1972 * t9258 / 2.0_f64);
    let t83672 = 12.0_f64 * t2314 * t22561;
    (t83666, t83672)
}
