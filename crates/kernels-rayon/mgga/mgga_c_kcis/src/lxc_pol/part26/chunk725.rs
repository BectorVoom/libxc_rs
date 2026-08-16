//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 725/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk725(t1884: f64, t637: f64, t2132: f64, t448: f64, t1885: f64, t2272: f64, t1598: f64, t5742: f64) -> (f64, f64, f64, f64) {
    let t8133 = t1884 * t637;
    let t8136 = t448 * t2132;
    let t8141 = t1885 * t2272;
    let t8144 = t5742 * t1598;
    (t8133, t8136, t8141, t8144)
}
