//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 625/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk625(t251: f64, t3353: f64, t388: f64, t382: f64, t1195: f64, t1199: f64, t2865: f64, t41: f64, t359: f64, t375: f64, t1175: f64, t1179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3354 = t3353 * t251;
    let t3355 = t3354 * t388;
    let t3356 = t382 * t3355;
    let t3358 = t1195 * t1199;
    let t3359 = t382 * t3358;
    let t3361 = t2865 * t41;
    let t3362 = t3361 * t359;
    let t3363 = t375 * t3362;
    let t3365 = t1175 * t1179;
    (t3354, t3355, t3356, t3358, t3359, t3361, t3362, t3363, t3365)
}
