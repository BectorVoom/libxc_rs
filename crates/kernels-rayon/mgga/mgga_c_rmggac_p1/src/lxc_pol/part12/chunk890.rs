//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 890/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk890(t2305: f64, t35658: f64, t7255: f64, t8497: f64, t35654: f64, t1986: f64, t5160: f64, t675: f64, t2191: f64, t8587: f64, t26857: f64, t7518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39401 = t35658 * t2305;
    let t39403 = t7255 * t8497;
    let t39405 = t35654 * t2305;
    let t39406 = 0.19863479950205658386e-4_f64 * t39405;
    let t39418 = t675 * t1986 * t5160;
    let t39420 = t2191 * t8587;
    let t39423 = t26857 * t7518;
    (t39401, t39403, t39406, t39418, t39420, t39423)
}
