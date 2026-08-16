//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2073/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2073(t562: f64, t91005: f64, t22751: f64, t26385: f64, t26389: f64, t26467: f64, t6914: f64, t26426: f64, t81046: f64, t22690: f64, t7732: f64, t81195: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91006 = t91005 * t562;
    let t91010 = t22751 * t26385;
    let t91011 = 0.76763589786250567036e-1_f64 * t91010;
    let t91064 = t22751 * t26389;
    let t91065 = 0.76763589786250567036e-1_f64 * t91064;
    let t91076 = t6914 * t26467;
    let t91077 = 0.38381794893125283518e-1_f64 * t91076;
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    (t91006, t91011, t91065, t91077, t91078, t91081)
}
