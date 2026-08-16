//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 364/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk364(t524: f64, t1581: f64, t2059: f64, t1312: f64, t2306: f64) -> (f64, f64, f64) {
    let t536 = 0.0_f64 < t524;
    let t2321 = t1581 * t2059;
    let t2322 = t1312 * t2321;
    let t2326 = piecewise3(t536, t2306, -t2306);
    (t2321, t2322, t2326)
}
