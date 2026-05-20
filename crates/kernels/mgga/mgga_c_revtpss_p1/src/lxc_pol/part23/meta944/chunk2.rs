//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3100/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100<F: Float>(t43771: F, t45106: F, t45107: F, t68255: F, t68257: F, t81156: F, t81158: F, t81162: F, t81167: F, t81399: F, t81401: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t81416: F) -> (F, F) {
    let t81678 = F::cast_from(0.68863333333333333332e0_f64) * t68255 - F::cast_from(0.45908888888888888888e0_f64) * t68257 - F::cast_from(0.30872592592592592592e0_f64) * t43771 + F::cast_from(0.34431666666666666667e0_f64) * t81156 - F::new(0.103295e1) * t81158 + F::cast_from(0.17215833333333333334e1_f64) * t81162 + F::cast_from(0.68863333333333333334e1_f64) * t81167 + F::new(0.3529725e1) * t81399 + t45106 + t45107 + F::new(0.6311625e0) * t81401;
    let t81691 = -F::new(0.61977e1) * t81171 - F::new(0.123954e2) * t81175 - F::new(0.103295e1) * t81179 - F::cast_from(0.34431666666666666667e0_f64) * t81184 - F::new(0.103295e1) * t81188 + F::new(0.929655e1) * t81192 + F::new(0.123954e2) * t81196 + F::new(0.309885e1) * t81200 + F::new(0.309885e1) * t81204 + F::new(0.103295e1) * t81209 - F::cast_from(0.15302962962962962963e1_f64) * t81214 + F::cast_from(0.13892666666666666667e0_f64) * t81416;
    (t81678, t81691)
}
