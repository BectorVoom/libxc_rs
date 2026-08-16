//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1238/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1238(t28: f64, t265: f64, t504: f64, t23788: f64, t33476: f64, t25927: f64, t33483: f64, t1649: f64, t1914: f64, t33512: f64, t1409: f64, t1877: f64, t24191: f64, t2522: f64, t26744: f64, t26756: f64, t31434: f64, t33065: f64, t33466: f64, t52: f64, t7114: f64, t7649: f64, t7656: f64, t8566: f64, t8586: f64, t8591: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t33531 = t23788 * t33476;
    let t33537 = t25927 * t33483;
    let t33539 = t1649 * t1914;
    let t33547 = piecewise3(t505, 0.0_f64, t33512);
    let t33552 = piecewise3(t401, 3.0_f64 / 2.0_f64 * t2522 * t8566 * t7649 + t1877 * t33466 * t28 / 2.0_f64 - t1877 * t31434 * t7656 / 2.0_f64 + t1877 * t8566 * t1649 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t33531 - t1877 * t26744 * t8586 / 2.0_f64 + t26756 * t33537 - t1877 * t7114 * t33539 / 2.0_f64 - t1877 * t7114 * t33065 / 2.0_f64, -t8591 * t1409 / 2.0_f64 + t33547 * t52 / 2.0_f64);
    (t33531, t33537, t33539, t33547, t33552)
}
