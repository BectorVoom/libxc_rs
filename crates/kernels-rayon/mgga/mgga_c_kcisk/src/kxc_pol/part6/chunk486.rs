//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 486/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk486(t3805: f64, t721: f64, t140: f64, t3737: f64, t673: f64) -> (f64, f64, f64) {
    let t4808 = t3805 * t721;
    let t4809 = 0.55273148148148148147e-3_f64 * t4808;
    let t4811 = t140 * t3737 * t673;
    (t4808, t4809, t4811)
}
