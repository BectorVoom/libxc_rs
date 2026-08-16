//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 887/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk887(t34960: f64, t356: f64, t638: f64, t639: f64, t8849: f64, t34750: f64, t34755: f64, t577: f64, t2392: f64, t866: f64, t262: f64, t8620: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39364 = 0.2927036860455597649e0_f64 * t34960;
    let t39367 = t638 * t639 * t8849 * t356;
    let t39370 = t34755 * t577 * t34750;
    let t39372 = t2392 * t866;
    let t39373 = t262 * t39372;
    let t39374 = t8620 * t39373;
    (t39364, t39367, t39370, t39372, t39373, t39374)
}
