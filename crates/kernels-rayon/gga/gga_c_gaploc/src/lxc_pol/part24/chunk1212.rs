//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1212/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1212(t25260: f64, t2558: f64, t9647: f64, t10691: f64, t1850: f64, t1895: f64, t481: f64, t686: f64, t10809: f64, t169: f64, t7305: f64, t10704: f64, t21665: f64) -> (f64, f64, f64, f64, f64) {
    let t32158 = t9647 * t25260 * t2558;
    let t32159 = 0.32043859292259267849e-3_f64 * t32158;
    let t32160 = t1850 * t10691;
    let t32161 = 0.85450291446024714264e-3_f64 * t32160;
    let t32163 = t481 * t1895 * t686;
    let t32167 = 0.1845726295234133828e0_f64 * t32163 * t10809 * t169 * t7305;
    let t32168 = t21665 * t10704;
    (t32159, t32161, t32163, t32167, t32168)
}
