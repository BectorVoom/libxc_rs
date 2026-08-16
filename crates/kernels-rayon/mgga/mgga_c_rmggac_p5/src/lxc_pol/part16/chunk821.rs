//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 821/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk821(t1173: f64, t674: f64, t9085: f64, t2868: f64, t7779: f64, t2186: f64, t8597: f64, t1982: f64, t7428: f64, t8688: f64, t1627: f64, t2064: f64, t3928: f64) -> (f64, f64, f64, f64, f64) {
    let t40359 = t9085 * t1173 * t674;
    let t40458 = t2868 * t7779;
    let t40479 = t2186 * t8597;
    let t40505 = t8688 * t7428 * t1982;
    let t40516 = t3928 * t2064 * t1627;
    (t40359, t40458, t40479, t40505, t40516)
}
