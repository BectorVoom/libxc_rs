//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1674/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1674<F: Float>(t77804: F, t88085: F, t88093: F, t88104: F, t88108: F, t88114: F, t88122: F, t88130: F, t88220: F, t88222: F, t88224: F, t88226: F, t88229: F, t88232: F) -> F {
    let t88412 = -F::new(0.379785e1) * t88220 - F::new(0.46074375e0) * t88222 + F::new(0.614325e0) * t88224 + F::new(0.85451625e1) * t88226 - F::cast_from(0.21908444444444444444e0_f64) * t88229 + F::cast_from(0.65725333333333333332e0_f64) * t88232 + F::new(0.71752e1) * t88085 + F::new(0.17938e1) * t88093 - F::cast_from(0.88582716049382716048e0_f64) * t88104 - F::cast_from(0.29896666666666666667e0_f64) * t88108 + F::cast_from(0.39862222222222222223e1_f64) * t88114 - F::cast_from(0.71752000000000000002e1_f64) * t88122 - F::cast_from(0.59793333333333333333e0_f64) * t88130 - F::cast_from(0.13145066666666666666e1_f64) * t77804;
    t88412
}
