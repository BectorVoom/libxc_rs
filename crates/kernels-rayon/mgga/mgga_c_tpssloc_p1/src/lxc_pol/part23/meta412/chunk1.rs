//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1230/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230(t12923: f64, t4194: f64, t5398: f64, t20800: f64, t262: f64, t10143: f64, t20778: f64, t13115: f64, t16586: f64, t21038: f64, t225: f64, t21061: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67230 = t4194 * t12923 * t5398;
    let t67235 = t262 * t20800;
    let t67239 = t20778 * t10143;
    let t67243 = t13115 * t16586;
    let t67305 = t21038 * t225;
    let t67339 = t21061 * t225;
    (t67230, t67235, t67239, t67243, t67305, t67339)
}
