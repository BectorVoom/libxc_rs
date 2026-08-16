//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 545/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk545<F: Float>(t1614: F, t945: F, t1622: F, t953: F, t2848: F, t2906: F, t2950: F, t2957: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F) -> (F, F, F) {
    let t4647 = t1614 * t945;
    let t4652 = t1622 * t953;
    let t4669 = -F::cast_from(0.17648625e1_f64) * t4599 + F::cast_from(0.3529725e1_f64) * t4607 + t2950 + F::cast_from(0.17215833333333333333e0_f64) * t2848 + F::cast_from(0.17215833333333333333e0_f64) * t4571 - F::cast_from(0.34431666666666666667e0_f64) * t4576 + F::cast_from(0.103295e1_f64) * t4581 - F::cast_from(0.516475e0_f64) * t4585 + F::cast_from(0.31558125e0_f64) * t4615 + F::cast_from(0.6311625e0_f64) * t4617 + t2957 + F::cast_from(0.69463333333333333333e-1_f64) * t2906 + F::cast_from(0.69463333333333333333e-1_f64) * t4620 - F::cast_from(0.34731666666666666667e-1_f64) * t4623 + F::cast_from(0.20839e0_f64) * t4626 - F::cast_from(0.104195e0_f64) * t4629;
    (t4647, t4652, t4669)
}
