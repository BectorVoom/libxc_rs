//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3669/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3669<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F, t68507: F, t68515: F, t68518: F, t68521: F, t68524: F) -> F {
    let t69312 = -F::cast_from(0.61745185185185185187e0_f64) * t58153 + F::cast_from(0.9261777777777777778e-1_f64) * t58158 + F::cast_from(0.4630888888888888889e-1_f64) * t58160 + F::cast_from(0.27785333333333333334e0_f64) * t58162 + F::cast_from(0.18523555555555555556e0_f64) * t68507 - F::cast_from(0.15436296296296296297e0_f64) * t58165 - F::cast_from(0.15302962962962962963e0_f64) * t43865 - F::cast_from(0.10712074074074074074e1_f64) * t43888 + F::cast_from(0.22954444444444444444e0_f64) * t43890 + F::cast_from(0.45908888888888888888e0_f64) * t43892 - F::cast_from(0.83356000000000000001e0_f64) * t68515 + F::cast_from(0.250068e1_f64) * t68518 - F::cast_from(0.55570666666666666668e0_f64) * t58186 - F::cast_from(0.3529725e1_f64) * t68521 - F::cast_from(0.20839e0_f64) * t68524;
    t69312
}
