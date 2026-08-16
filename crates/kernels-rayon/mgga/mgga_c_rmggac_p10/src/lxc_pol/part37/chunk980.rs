//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 980/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk980(t77732: f64, t2144: f64, t3351: f64, t3352: f64, t9555: f64, t1971: f64, t7190: f64, t9558: f64, t7262: f64, t9541: f64, t15624: f64, t498: f64, t515: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64, f64) {
    let t77733 = 0.12769379967989351819e-4_f64 * t77732;
    let t77736 = t3351 * t3352 * t2144 * t9555;
    let t77737 = 0.38308139903968055457e-4_f64 * t77736;
    let t77740 = t3351 * t1971 * t7190 * t9558;
    let t77741 = 0.51077519871957407276e-4_f64 * t77740;
    let t77744 = t3351 * t1971 * t7262 * t9541;
    let t77745 = 0.25538759935978703638e-4_f64 * t77744;
    let t77749 = t7230 * t7231 * t515 * t15624 * t498;
    (t77733, t77737, t77741, t77745, t77749)
}
