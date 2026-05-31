//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1207/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1207<F: Float>(t1016: F, t21438: F, t10295: F, t17288: F, t17277: F, t3366: F, t2358: F, t27232: F, t10281: F, t501: F, t605: F, t10629: F, t5227: F) -> (F, F, F, F, F, F) {
    let t32091 = t21438 * t1016;
    let t32093 = F::cast_from(12.0_f64) * t17288 * t10295;
    let t32095 = F::cast_from(2.0_f64) * t17277 * t3366;
    let t32099 = F::cast_from(4.0_f64) * t27232 * t2358;
    let t32100 = t10281 * t501;
    let t32102 = F::cast_from(2.0_f64) * t32100 * t605;
    let t32104 = F::cast_from(0.34180116578409885704e-2_f64) * t5227 * t10629;
    (t32091, t32093, t32095, t32099, t32102, t32104)
}
