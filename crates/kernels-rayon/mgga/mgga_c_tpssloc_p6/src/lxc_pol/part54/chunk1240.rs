//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1240/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1240(t191: f64, t192: f64, t8107: f64, t3701: f64, t7939: f64, t33199: f64, t33208: f64, t33213: f64, t33216: f64, t33218: f64, t33224: f64, t33227: f64, t33230: f64, t33233: f64, t33236: f64, t33238: f64, t33239: f64, t33337: f64) -> (f64, f64, f64) {
    let t33746 = t8107 * t191 * t192;
    let t33899 = t3701 * t7939;
    let t34104 = -t33199 - t33208 - t33213 - t33216 - t33218 + t33224 - t33227 - t33230 - t33233 - t33236 - t33238 + t33239 + t33337;
    (t33746, t33899, t34104)
}
