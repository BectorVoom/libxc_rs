//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1253/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1253(t80825: f64, t557: f64, t6546: f64, t1365: f64, t1878: f64, t22813: f64, t6924: f64, t80782: f64, t22843: f64, t281: f64, t6597: f64, t154: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80826 = 0.10173934535723378495e0_f64 * t80825;
    let t80827 = t6546 * t557;
    let t80830 = t1878 * t1365;
    let t80836 = t22813 * t6924 * t80782;
    let t80840 = t6597 * t22843 * t281;
    let t80845 = t8705 * t154;
    (t80826, t80827, t80830, t80836, t80840, t80845)
}
