//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2211/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2211(t1880: f64, t23237: f64, t28294: f64, t22986: f64, t28267: f64, t82159: f64, t25054: f64, t86873: f64, t6552: f64, t6555: f64, t98133: f64, t25216: f64, t25224: f64) -> (f64, f64, f64, f64, f64) {
    let t98189 = t1880 * t23237 * t28294;
    let t98192 = t22986 * t82159 * t28267;
    let t98196 = t22986 * t86873 * t25054;
    let t98199 = t6552 * t98133 * t6555;
    let t98202 = t1880 * t25224 * t25216;
    (t98189, t98192, t98196, t98199, t98202)
}
