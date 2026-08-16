//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1217/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1217(t28: f64, t265: f64, t504: f64, t33043: f64, t1409: f64, t33073: f64, t52: f64, t8435: f64, t15899: f64, t8493: f64, t1983: f64, t1458: f64, t1868: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t33074 = piecewise3(t505, 0.0_f64, t33043);
    let t33079 = piecewise3(t401, t33073, -t8435 * t1409 / 2.0_f64 + t33074 * t52 / 2.0_f64);
    let t33082 = t8493 * t15899;
    let t33084 = 2.0_f64 * t1983 * t33082;
    let t33085 = t1868 * t1458;
    (t33074, t33079, t33082, t33084, t33085)
}
