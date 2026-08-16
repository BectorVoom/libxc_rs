//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 565/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk565(t14639: f64, t675: f64, t2186: f64, t3219: f64, t14144: f64, t1356: f64, t14441: f64, t14156: f64, t14171: f64, t14175: f64, t14186: f64, t14190: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14640 = t675 * t14639;
    let t14641 = 0.42564599893297839398e-5_f64 * t14640;
    let t14642 = t2186 * t3219;
    let t14649 = 0.14967802127329760705e-1_f64 * t14144;
    let t14650 = t1356 * t14441;
    let t14651 = 0.39914139006212695214e-1_f64 * t14650;
    let t14653 = 0.10227998120342003148e-1_f64 * t14156;
    let t14655 = 0.44903406381989282115e-1_f64 * t14171;
    let t14656 = 0.14967802127329760705e-1_f64 * t14175;
    let t14659 = 0.85129199786595678799e-5_f64 * t14186;
    let t14660 = 0.2553875993597870364e-4_f64 * t14190;
    (t14641, t14642, t14649, t14651, t14653, t14655, t14656, t14659, t14660)
}
