//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 749/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk749(t107: f64, t10809: f64, t787: f64, t3487: f64, t5241: f64, t16687: f64, t19: f64, t60: f64, t822: f64, t16692: f64, t201: f64, t2536: f64, t2925: f64) -> (f64, f64, f64, f64, f64) {
    let t33294 = t787 * t10809 * t107;
    let t33308 = t5241 * t3487;
    let t33331 = t822 * t16687 * t19 * t60;
    let t33332 = t201 * t16692;
    let t33348 = t2536 * t2925;
    (t33294, t33308, t33331, t33332, t33348)
}
