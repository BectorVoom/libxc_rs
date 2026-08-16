//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2328/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2328(t28: f64, t265: f64, t504: f64, t100674: f64, t100716: f64, t100763: f64, t100803: f64, t100624: f64, t1409: f64, t16558: f64, t1972: f64, t25950: f64, t28803: f64, t3966: f64, t52: f64, t5398: f64, t607: f64, t6856: f64, t7664: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t100805 = t100674 + t100716 + t100763 + t100803;
    let t100806 = piecewise3(t505, 0.0_f64, t100624);
    let t100818 = piecewise3(t401, t100805, t100806 * t52 / 2.0_f64 - t28803 * t607 / 2.0_f64 - t25950 * t1409 - t7664 * t3966 - t6856 * t5398 / 2.0_f64 - t1972 * t16558 / 2.0_f64);
    t100818
}
