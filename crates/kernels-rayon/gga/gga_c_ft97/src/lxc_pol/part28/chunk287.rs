//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 287/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk287(t3157: f64, t457: f64, t91: f64, t2981: f64, t3006: f64, t1549: f64, t1552: f64, t1832: f64, t2986: f64, t2990: f64, t2995: f64, t3003: f64, t3011: f64, t3016: f64, t3106: f64, t3121: f64) -> (f64, f64) {
    let t3159 = t91 * t457 * t3157;
    let t3161 = t2981 / 27.0_f64;
    let t3166 = t3006 / 9.0_f64;
    let t3170 = -t3121 / 12.0_f64 + t3159 / 6.0_f64 + t1832 + t1549 + t1552 + t3161 - 2.0_f64 / 27.0_f64 * t2986 + t2990 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t2995 - 2.0_f64 / 9.0_f64 * t3003 + t3166 + t3011 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t3016 - t3106 / 3.0_f64;
    (t3159, t3170)
}
