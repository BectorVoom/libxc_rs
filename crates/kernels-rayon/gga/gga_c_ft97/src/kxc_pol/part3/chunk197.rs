//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 197/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk197(t637: f64, t639: f64, t643: f64, t629: f64, t631: f64, t634: f64, t184: f64, t21: f64, t231: f64, t240: f64, t247: f64, t342: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t645 = t637 * t639 * t643;
    let t648 = t629 + t631 * t634 / 6.0_f64 + t631 * t645 / 2.0_f64;
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t657 = t231 * t240;
    let t661 = t247 - t342 * t343 * t657 / 4.0_f64;
    (t645, t648, t649, t650, t657, t661)
}
