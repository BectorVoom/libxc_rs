//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 799/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk799(t1843: f64, t28302: f64, t7064: f64, t28703: f64, t883: f64, t2537: f64, t9647: f64, t2558: f64, t28431: f64, t12629: f64, t731: f64, t12604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40836 = t7064 * t1843 * t28302;
    let t40848 = t883 * t28703;
    let t40850 = t9647 * t2537 * t40848;
    let t40853 = t9647 * t28431 * t2558;
    let t40877 = t731 * t12629;
    let t40890 = t731 * t12604;
    (t40836, t40848, t40850, t40853, t40877, t40890)
}
