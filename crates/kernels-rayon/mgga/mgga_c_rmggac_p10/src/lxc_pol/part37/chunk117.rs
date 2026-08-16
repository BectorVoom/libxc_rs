//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 117/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk117(t446: f64, t489: f64, t490: f64, t467: f64, t479: f64, t487: f64, t488: f64) -> (f64, f64) {
    let t492 = t489 * t490 * t446;
    let t495 = -0.27439556402611977244e-1_f64 * t467 * t479 - t487 - 0.54879112805223954488e-1_f64 * t488 * t492;
    (t492, t495)
}
