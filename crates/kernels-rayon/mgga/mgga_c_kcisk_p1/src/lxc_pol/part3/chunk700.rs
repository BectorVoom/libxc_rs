//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 700/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk700(t10924: f64, t608: f64, t1724: f64, t4859: f64, t4910: f64, t620: f64, t342: f64, t569: f64, t969: f64) -> (f64, f64, f64, f64) {
    let t10925 = t608 * t10924;
    let t10926 = t4859 * t1724;
    let t10928 = 1.0_f64 / t4910 / t620;
    let t10929 = t10926 * t10928;
    let t10933 = t342 * t969 * t569;
    (t10925, t10926, t10929, t10933)
}
