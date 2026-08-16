//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 980/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk980(t12161: f64, t13045: f64, t14385: f64, t1445: f64, t2197: f64, t2949: f64, t44117: f64, t44133: f64, t45913: f64, t45915: f64, t45922: f64, t45931: f64, t45933: f64, t45939: f64, t45946: f64, t45947: f64, t45950: f64, t45953: f64, t45958: f64, t47496: f64, t47500: f64, t47517: f64, t47555: f64, t47558: f64, t813: f64) -> f64 {
    let t50302 = t45913 + 0.11916829983950142223e0_f64 * t47517 + t45915 - 0.63904876589867916127e-1_f64 * t44117 - t45922 + t45931 - t45933 + t45939 - t45946 - 0.21450293971110256002e1_f64 * t47500 * t13045 + 0.23005755572352449806e2_f64 * t2197 * t14385 + 0.44688112439813033337e-1_f64 * t45947 - 0.89376224879626066674e-1_f64 * t45950 - t45953 + 0.63904876589867916127e-1_f64 * t44133 + t47555 - t47558 - 0.92023022289409799224e1_f64 * t813 * t1445 * t2949 * t12161 - 0.21450293971110256002e1_f64 * t47496 * t13045 + t45958;
    t50302
}
