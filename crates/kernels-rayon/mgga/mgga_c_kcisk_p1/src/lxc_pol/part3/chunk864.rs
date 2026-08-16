//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 864/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk864(t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12948: f64, t12954: f64, t12959: f64, t12975: f64, t12985: f64, t12989: f64, t1173: f64) -> (f64, f64) {
    let t12992 = -t12975 - 4.0_f64 / 9.0_f64 * t12929 + 2.0_f64 / 9.0_f64 * t12933 - 2.0_f64 / 3.0_f64 * t12948 + t12931 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t12922 + 4.0_f64 / 3.0_f64 * t12954 - 2.0_f64 / 3.0_f64 * t12985 - 2.0_f64 * t12959 + 2.0_f64 * t12989 - t12927 / 3.0_f64;
    let t12993 = t1173 * t12992;
    (t12992, t12993)
}
