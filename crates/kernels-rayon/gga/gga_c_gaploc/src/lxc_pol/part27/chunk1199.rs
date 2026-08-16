//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1199/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1199(t32160: f64, t1895: f64, t481: f64, t686: f64, t10809: f64, t169: f64, t7305: f64, t10704: f64, t21665: f64, t1843: f64, t24474: f64, t7064: f64) -> (f64, f64, f64, f64, f64) {
    let t32161 = 0.85450291446024714264e-3_f64 * t32160;
    let t32163 = t481 * t1895 * t686;
    let t32167 = 0.1845726295234133828e0_f64 * t32163 * t10809 * t169 * t7305;
    let t32168 = t21665 * t10704;
    let t32169 = 0.64087718584518535698e-3_f64 * t32168;
    let t32171 = t7064 * t1843 * t24474;
    (t32161, t32163, t32167, t32169, t32171)
}
