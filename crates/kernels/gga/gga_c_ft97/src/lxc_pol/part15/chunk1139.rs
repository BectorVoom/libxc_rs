//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1139/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1139<F: Float>(t2345: F, t2348: F, t88239: F, t89: F, t2361: F, t666: F, t42759: F, t80819: F, t80821: F, t88218: F, t88221: F, t88225: F, t88229: F, t88233: F, t88237: F, t89022: F, t89027: F, t89030: F, t89034: F) -> (F, F, F) {
    let t89038 = t89 * t2345 * t2348 * t88239;
    let t89042 = t89 * t666 * t2361 * t88239;
    let t89044 = -t88218 + t42759 + F::new(4.0) / F::new(3.0) * t88221 - F::new(4.0) * t88225 - F::new(8.0) / F::new(3.0) * t88229 - F::new(3.0) / F::new(4.0) * t88233 - F::new(15.0) / F::new(16.0) * t88237 + t89022 / F::new(2.0) - F::new(4.0) / F::new(3.0) * t80819 - F::new(4.0) / F::new(3.0) * t80821 - F::new(20.0) / F::new(9.0) * t89027 - F::new(12.0) * t89030 - F::new(8.0) * t89034 - F::new(2.0) / F::new(3.0) * t89038 + F::new(2.0) * t89042;
    (t89038, t89042, t89044)
}
