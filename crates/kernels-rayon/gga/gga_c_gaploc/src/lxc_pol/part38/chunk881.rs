//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 881/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk881(t2508: f64, t35719: f64, t954: f64, t44707: f64, t688: f64, t779: f64, t1843: f64, t35500: f64, t7064: f64, t35550: f64, t5539: f64, t13545: f64, t7137: f64) -> (f64, f64, f64, f64, f64) {
    let t45044 = 0.15381052460284448567e-1_f64 * t2508 * t954 * t35719;
    let t45048 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t44707 * t688;
    let t45051 = t7064 * t1843 * t35500;
    let t45052 = 0.32043859292259267849e-3_f64 * t45051;
    let t45054 = t7064 * t5539 * t35550;
    let t45057 = 0.71778244814660759981e-1_f64 * t7137 * t13545;
    (t45044, t45048, t45052, t45054, t45057)
}
