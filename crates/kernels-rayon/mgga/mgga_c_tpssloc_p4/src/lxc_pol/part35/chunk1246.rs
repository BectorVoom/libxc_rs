//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1246/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1246(t2239: f64, t5385: f64, t1176: f64, t1714: f64, t111: f64, t20292: f64, t21038: f64, t225: f64, t21061: f64, t21036: f64, t20856: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55921 = t5385 * t2239;
    let t64825 = t1176 * t1714;
    let t67001 = t20292 * t111;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    let t67350 = t252 * t20856;
    (t55921, t64825, t67001, t67305, t67339, t67344, t67350)
}
