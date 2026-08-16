//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2124/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124(t28831: f64, t83886: f64, t6287: f64, t652: f64, t6534: f64, t26168: f64, t7685: f64, t19924: f64, t24995: f64, t8945: f64, t19456: f64, t7468: f64) -> (f64, f64, f64, f64, f64) {
    let t96755 = 6.0_f64 * t83886 * t28831;
    let t96758 = 2.0_f64 * t652 * t6287 * t6534;
    let t96760 = 6.0_f64 * t7685 * t26168;
    let t96763 = 12.0_f64 * t24995 * t8945 * t19924;
    let t96765 = 4.0_f64 * t19456 * t7468;
    (t96755, t96758, t96760, t96763, t96765)
}
