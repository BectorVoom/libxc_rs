//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 339/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk339(t1162: f64, t2077: f64, t321: f64, t1161: f64) -> (f64, f64, f64) {
    let t2079 = -t1162 - 0.17808333333333333333e-1_f64 * t2077;
    let t2081 = 0.62182e-1_f64 * t2079 * t321;
    let t2083 = -t1161 / 3.0_f64 - t2077 / 3.0_f64;
    (t2079, t2081, t2083)
}
