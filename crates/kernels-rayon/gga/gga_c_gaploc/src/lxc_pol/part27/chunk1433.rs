//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1433/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1433(t12215: f64, t1457: f64, t2103: f64, t28633: f64, t28636: f64, t28645: f64, t28675: f64, t28678: f64, t28681: f64, t28683: f64, t33412: f64, t33416: f64, t33419: f64, t33421: f64, t33429: f64, t33453: f64, t39091: f64, t39095: f64, t5771: f64) -> f64 {
    let t39136 = t33412 + 0.71500979903700853338e0_f64 * t2103 * t1457 * t39091 + 0.14300195980740170668e1_f64 * t5771 * t12215 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t39095 - t28633 + t28636 + t33416 + t33419 + t33421 + t28645 + t33429 + t28675 + t28678 + t28681 - 0.53964118009221795842e0_f64 * t28683 - t33453;
    t39136
}
