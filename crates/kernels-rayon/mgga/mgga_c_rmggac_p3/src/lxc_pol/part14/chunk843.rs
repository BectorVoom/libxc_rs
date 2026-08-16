//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 843/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk843(t1661: f64, t2010: f64, t7359: f64, t7335: f64, t8349: f64, t2415: f64, t4025: f64, t2011: f64, t291: f64, t5354: f64, t7508: f64, t8533: f64) -> (f64, f64, f64, f64, f64) {
    let t38755 = t2010 * t7359 * t1661;
    let t38757 = t7335 * t8349;
    let t38760 = t2010 * t2415 * t4025;
    let t38764 = t2010 * t2011 * t5354 * t291;
    let t38775 = t7508 * t8533;
    (t38755, t38757, t38760, t38764, t38775)
}
