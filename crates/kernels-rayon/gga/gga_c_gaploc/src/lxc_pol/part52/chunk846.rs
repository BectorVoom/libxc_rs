//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 846/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk846(t1843: f64, t35500: f64, t7064: f64, t35550: f64, t5539: f64, t13545: f64, t7137: f64, t13495: f64, t7129: f64, t2508: f64, t2717: f64, t3616: f64) -> (f64, f64, f64, f64, f64) {
    let t45051 = t7064 * t1843 * t35500;
    let t45052 = 0.32043859292259267849e-3_f64 * t45051;
    let t45054 = t7064 * t5539 * t35550;
    let t45057 = 0.71778244814660759981e-1_f64 * t7137 * t13545;
    let t45059 = 0.76905262301422242837e-2_f64 * t7129 * t13495;
    let t45062 = 0.76905262301422242837e-2_f64 * t2508 * t2717 * t3616;
    (t45052, t45054, t45057, t45059, t45062)
}
