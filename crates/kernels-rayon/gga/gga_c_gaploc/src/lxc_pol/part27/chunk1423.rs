//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1423/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1423(t28283: f64, t28289: f64, t28290: f64, t28296: f64, t28307: f64, t28312: f64, t33009: f64, t33013: f64, t33018: f64, t33021: f64, t33024: f64, t33030: f64, t33033: f64, t33041: f64, t33048: f64, t33055: f64) -> f64 {
    let t38998 = -t33009 - t33013 + t33018 + t33021 + t33024 - 0.15337170381568299871e1_f64 * t28283 - t28289 - 0.76685851907841499354e0_f64 * t28290 + 0.15337170381568299871e1_f64 * t28296 - 0.76685851907841499354e0_f64 * t28307 + t28312 + t33030 + t33033 + t33041 - t33048 - t33055;
    t38998
}
