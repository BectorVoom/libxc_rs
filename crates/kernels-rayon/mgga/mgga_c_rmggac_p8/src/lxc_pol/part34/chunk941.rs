//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 941/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk941(t73881: f64, t73896: f64, t3219: f64, t38472: f64, t1971: f64, t2447: f64, t495: f64, t515: f64, t7230: f64, t73902: f64, t73909: f64, t73912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76755 = 0.85129199786595678799e-5_f64 * t73881;
    let t76757 = 0.85129199786595678799e-5_f64 * t73896;
    let t76758 = t38472 * t3219;
    let t76759 = 0.42564599893297839398e-5_f64 * t76758;
    let t76763 = t7230 * t1971 * t515 * t2447 * t495;
    let t76764 = 0.53205749866622299248e-5_f64 * t76763;
    let t76766 = 0.19709219354514038085e-5_f64 * t73902;
    let t76768 = 0.2627895913935205078e-5_f64 * t73909;
    let t76769 = 0.2627895913935205078e-5_f64 * t73912;
    (t76755, t76757, t76759, t76764, t76766, t76768, t76769)
}
