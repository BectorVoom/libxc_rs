//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1113/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1113(t10186: f64, t10233: f64, t10267: f64, t10274: f64, t13806: f64, t13813: f64, t13817: f64, t13825: f64, t13830: f64, t13832: f64, t13836: f64, t13840: f64, t2960: f64, t2986: f64, t4523: f64, t4532: f64, t4549: f64, t973: f64) -> f64 {
    let t13845 = 0.12345679012345679012e-3_f64 * t10233 + 0.55555555555555555554e-3_f64 * t2986 * t13806 - 0.49382716049382716048e-3_f64 * t10267 - 0.18518518518518518518e-3_f64 * t10274 - 0.16666666666666666666e-2_f64 * t973 * t13813 + 0.27777777777777777777e-3_f64 * t973 * t13817 + 0.44444444444444444444e-2_f64 * t2960 * t4549 - t13825 - 0.14814814814814814814e-2_f64 * t2960 * t4523 + t13830 - 0.55555555555555555554e-3_f64 * t2986 * t13832 + 0.11111111111111111111e-2_f64 * t2986 * t13836 - 0.74074074074074074072e-3_f64 * t2986 * t13840 + 0.14814814814814814814e-2_f64 * t10186 * t4532;
    t13845
}
