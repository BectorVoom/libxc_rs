//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 899/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk899(t1869: f64, t28969: f64, t22254: f64, t2537: f64, t10812: f64, t28368: f64, t5006: f64, t2364: f64, t8814: f64, t11179: f64, t2464: f64, t8514: f64) -> (f64, f64, f64, f64, f64) {
    let t28970 = t1869 * t28969;
    let t28972 = t22254 * t2537;
    let t28973 = t1869 * t28972;
    let t28977 = t10812 * t28368;
    let t28978 = t5006 * t28977;
    let t28991 = t2364 * t8814;
    let t28992 = t11179 * t28991;
    let t28995 = t8514 * t2464;
    (t28970, t28973, t28978, t28992, t28995)
}
