//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 615/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk615(t15517: f64, t2412: f64, t3219: f64, t1986: f64, t2472: f64, t675: f64, t2471: f64, t36: f64, t739: f64, t15281: f64, t2211: f64, t2367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15518 = 0.39914139006212695214e-1_f64 * t15517;
    let t15521 = t2412 * t3219;
    let t15522 = 0.42564599893297839398e-5_f64 * t15521;
    let t15523 = t1986 * t2472;
    let t15524 = t675 * t15523;
    let t15525 = 0.42564599893297839398e-5_f64 * t15524;
    let t15526 = t2471 * t36;
    let t15527 = t739 * t15526;
    let t15528 = 0.14967802127329760705e-1_f64 * t15527;
    let t15529 = 0.14967802127329760705e-1_f64 * t15281;
    let t15530 = t2211 * t2367;
    (t15518, t15522, t15523, t15525, t15526, t15528, t15529, t15530)
}
