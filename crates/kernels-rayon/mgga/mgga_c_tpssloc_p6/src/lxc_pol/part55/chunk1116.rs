//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1116/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1116(t25: f64, t1409: f64, t32907: f64, t33750: f64, t40: f64, t8678: f64, t7754: f64, t8690: f64, t1873: f64, t27921: f64, t24972: f64, t7769: f64, t7423: f64, t7467: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t33755 = piecewise3(t115, t32907, t8678 * t1409 / 2.0_f64 + t33750 * t40 / 2.0_f64);
    let t33758 = t8690 * t7754;
    let t33774 = t27921 * t1873;
    let t33776 = t24972 * t7769;
    let t33778 = t7423 * t7467;
    (t33755, t33758, t33774, t33776, t33778)
}
