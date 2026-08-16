//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 828/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk828(t26: f64, t7834: f64, t797: f64, t838: f64, t40331: f64, t793: f64, t558: f64, t7817: f64, t305: f64, t38381: f64, t262: f64, t40802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    let t40932 = t838 * t40927;
    let t40944 = t793 * t40331;
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40965 = t262 * t40802;
    (t40928, t40932, t40944, t40948, t40949, t40951, t40965)
}
