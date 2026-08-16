//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2870/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870<F: Float>(t2439: F, t4628: F, t1606: F, t9303: F, t52115: F, t916: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41441: F, t52112: F) -> (F, F, F, F) {
    let t52126 = t2439 * t4628;
    let t52127 = F::cast_from(0.27595e0_f64) * t52126;
    let t52128 = t9303 * t1606;
    let t52130 = t916 * t52115;
    let t52134 = -F::cast_from(0.60385000000000000002e0_f64) * t41365 + F::cast_from(0.20128333333333333334e0_f64) * t41367 + F::cast_from(0.60385000000000000002e0_f64) * t41308 - F::cast_from(0.40256666666666666667e0_f64) * t41330 - F::cast_from(0.26837777777777777778e0_f64) * t41332 + F::cast_from(0.10064166666666666667e0_f64) * t41334 + F::cast_from(0.11182407407407407408e0_f64) * t41336 - t52127 + F::cast_from(0.24528888888888888889e0_f64) * t52128 + F::cast_from(0.258925e1_f64) * t52130 - F::cast_from(0.543465e1_f64) * t52112 + F::cast_from(0.73586666666666666668e0_f64) * t41441;
    (t52126, t52128, t52130, t52134)
}
