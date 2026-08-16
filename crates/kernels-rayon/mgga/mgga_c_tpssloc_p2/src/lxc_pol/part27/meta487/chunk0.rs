//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1868/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1868(t28: f64, t265: f64, t504: f64, t23772: f64, t1972: f64, t2250: f64, t23820: f64, t52: f64, t607: f64, t6856: f64, t23780: f64, t1873: f64, t3652: f64, t652: f64, t6876: f64, t7000: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t23821 = piecewise3(t505, 0.0_f64, t23772);
    let t23828 = piecewise3(t401, t23820, t23821 * t52 / 2.0_f64 - t6856 * t607 - t1972 * t2250 / 2.0_f64);
    let t23829 = t23780 + t23828;
    let t23831 = t3652 * t1873;
    let t23833 = 2.0_f64 * t652 * t23831;
    let t23835 = 2.0_f64 * t6876 * t7000;
    (t23821, t23829, t23831, t23833, t23835)
}
