//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 750/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk750(t1022: f64, t7275: f64, t32356: f64, t739: f64, t10938: f64, t2021: f64, t10007: f64, t10627: f64, t1890: f64, t10600: f64, t1415: f64, t31585: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33360 = t7275 * t1022;
    let t33561 = t739 * t32356;
    let t33565 = t2021 * t10938;
    let t33601 = t10007 * t10627;
    let t33760 = t1890 * t32356;
    let t34264 = t1415 * t10600;
    let t34267 = t493 * t31585;
    (t33360, t33561, t33565, t33601, t33760, t34264, t34267)
}
