//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 557/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk557(t14516: f64, t2136: f64, t14328: f64, t14330: f64, t14336: f64, t14349: f64, t14351: f64, t14444: f64, t333: f64, t5266: f64, t13851: f64, t13856: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14517 = t14516 * t2136;
    let t14518 = 0.10227998120342003148e-1_f64 * t14517;
    let t14519 = 0.79828278012425390427e-1_f64 * t14328;
    let t14520 = 0.14967802127329760705e-1_f64 * t14330;
    let t14521 = 0.54549323308490683456e-1_f64 * t14336;
    let t14523 = 0.44903406381989282115e-1_f64 * t14349;
    let t14524 = 0.14967802127329760705e-1_f64 * t14351;
    let t14525 = t14444 * t333;
    let t14527 = 0.11974241701863808564e0_f64 * t5266 * t14525;
    let t14533 = 0.1276937996798935182e-4_f64 * t13851;
    let t14535 = 0.85129199786595678799e-5_f64 * t13856;
    (t14518, t14519, t14520, t14521, t14523, t14524, t14527, t14533, t14535)
}
