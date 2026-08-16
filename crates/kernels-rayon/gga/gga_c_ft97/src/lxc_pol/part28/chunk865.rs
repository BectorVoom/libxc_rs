//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 865/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk865(t34660: f64, t34693: f64, t34735: f64, t34782: f64, t1564: f64, t32423: f64, t925: f64, t1286: f64, t1310: f64, t32016: f64, t34553: f64, t34557: f64, t34560: f64, t34563: f64, t34566: f64, t34569: f64, t34577: f64, t34581: f64, t34585: f64, t34589: f64, t34614: f64, t34620: f64, t5501: f64, t6414: f64, t6418: f64, t6457: f64, t7162: f64, t7214: f64, t7218: f64, t7286: f64, t88: f64, t948: f64) -> (f64, f64, f64) {
    let t34784 = t34660 + t34693 + t34735 + t34782;
    let t34787 = t1564 * t32423 * t925;
    let t34790 = -t948 * t7286 - t32016 * t6418 / 18.0_f64 - t5501 * t34553 / 18.0_f64 + t5501 * t34557 / 9.0_f64 + 8.0_f64 * t34560 - 12.0_f64 * t34563 + 8.0_f64 * t34566 + 4.0_f64 * t34569 + t6414 * t7214 / 6.0_f64 + t6414 * t7218 / 3.0_f64 + t1286 * t34577 / 6.0_f64 + t1286 * t34581 / 6.0_f64 + t1286 * t34585 / 3.0_f64 + t1286 * t34589 / 3.0_f64 + t34614 * t1310 / 6.0_f64 + t7162 * t6457 / 6.0_f64 - t1286 * t34620 / 3.0_f64 - t88 * t34784 - t5501 * t34787 / 9.0_f64;
    (t34784, t34787, t34790)
}
