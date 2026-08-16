//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 593/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk593(t1986: f64, t2403: f64, t675: f64, t13862: f64, t572: f64, t3133: f64, t2318: f64, t305: f64, t13866: f64, t2281: f64, t2001: f64, t3141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15220 = t1986 * t2403;
    let t15221 = t675 * t15220;
    let t15223 = t13862 * t572;
    let t15224 = t3133 * t15223;
    let t15226 = t305 * t2318;
    let t15227 = t1986 * t15226;
    let t15228 = t13866 * t15227;
    let t15230 = t305 * t2281;
    let t15231 = t2001 * t15230;
    let t15232 = t3141 * t15231;
    (t15220, t15221, t15223, t15224, t15227, t15228, t15231, t15232)
}
