//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 253/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk253(t301: f64, t317: f64, t830: f64, t876: f64, t880: f64, t882: f64, t332: f64, t321: f64, t5: f64, t170: f64, t328: f64, t626: f64) -> (f64, f64, f64, f64) {
    let t885 = -t301 * t880 - t317 * t830 - 2.0_f64 * t876 + 2.0_f64 * t882;
    let t886 = t885 * t332;
    let t889 = t5 * t321;
    let t892 = t170 * t626 * t328 / 6.0_f64;
    (t885, t886, t889, t892)
}
