//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2932/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2932<F: Float>(t52597: F, t52598: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t77539: F, t77543: F, t77547: F, t77799: F, t52128: F, t52623: F, t63447: F, t63453: F, t63459: F, t77802: F, t77804: F, t77806: F, t77810: F, t77813: F, t77816: F, t77819: F) -> (F, F) {
    let t77935 = -F::new(0.929655e1) * t77539 + F::new(0.309885e1) * t77543 + F::new(0.309885e1) * t77547 - F::cast_from(0.20658999999999999999e1_f64) * t63338 + F::cast_from(0.68863333333333333332e0_f64) * t63340 + F::cast_from(0.5738611111111111111e0_f64) * t63342 + F::new(0.309885e1) * t63361 - F::new(0.20659e1) * t63371 + t52597 - t52598 + F::new(0.6311625e0) * t77799;
    let t77947 = F::new(0.3529725e1) * t77802 - F::new(0.41678e0) * t77804 + F::cast_from(0.69463333333333333333e-1_f64) * t77806 - t52623 + F::cast_from(0.92617777777777777779e0_f64) * t52128 + F::new(0.250068e1) * t77810 - F::new(0.187551e1) * t77813 + F::new(0.62517e0) * t77816 + F::new(0.62517e0) * t77819 + F::cast_from(0.51647499999999999999e0_f64) * t63447 - F::cast_from(0.45908888888888888888e0_f64) * t63453 + F::cast_from(0.13772666666666666667e1_f64) * t63459;
    (t77935, t77947)
}
