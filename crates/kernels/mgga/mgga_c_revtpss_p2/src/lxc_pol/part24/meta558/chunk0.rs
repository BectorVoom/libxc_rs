//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1669/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1669<F: Float>(t77804: F, t88085: F, t88093: F, t88104: F, t88108: F, t88114: F, t88122: F, t88130: F, t88220: F, t88222: F, t88224: F, t88226: F, t88229: F, t88232: F) -> F {
    let t88321 = -F::new(0.705945e1) * t88220 - F::new(0.94674375e0) * t88222 + F::new(0.1262325e1) * t88224 + F::cast_from(0.158837625e2_f64) * t88226 - F::cast_from(0.27785333333333333334e0_f64) * t88229 + F::new(0.83356e0) * t88232 + F::new(0.123954e2) * t88085 + F::new(0.309885e1) * t88093 - F::cast_from(0.15302962962962962963e1_f64) * t88104 - F::new(0.516475e0) * t88108 + F::cast_from(0.68863333333333333334e1_f64) * t88114 - F::new(0.123954e2) * t88122 - F::new(0.103295e1) * t88130 - F::new(0.166712e1) * t77804;
    t88321
}
