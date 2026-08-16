//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1862/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1862(t13265: f64, t87368: f64, t13333: f64, t25084: f64, t13076: f64, t23146: f64, t13084: f64, t25083: f64, t2617: f64, t4184: f64, t13244: f64, t25064: f64, t81788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87369 = t87368 * t13265;
    let t87371 = t25084 * t13333;
    let t87373 = t23146 * t13076;
    let t87375 = t25084 * t13084;
    let t87379 = t2617 * t25083 * t4184;
    let t87381 = t25084 * t13244;
    let t87387 = t81788 * t25064;
    (t87369, t87371, t87373, t87375, t87379, t87381, t87387)
}
