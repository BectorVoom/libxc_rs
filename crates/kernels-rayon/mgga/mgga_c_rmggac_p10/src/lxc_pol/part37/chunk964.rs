//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 964/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk964(t75024: f64, t75033: f64, t75037: f64, t1986: f64, t2464: f64, t7720: f64, t75051: f64, t75060: f64, t75077: f64, t75084: f64, t75088: f64, t75096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77428 = 0.638468998399467591e-4_f64 * t75024;
    let t77430 = 0.23268647941669485538e-4_f64 * t75033;
    let t77431 = 0.23268647941669485538e-4_f64 * t75037;
    let t77435 = t1986 * t2464;
    let t77436 = t7720 * t77435;
    let t77437 = 0.12769379967989351819e-4_f64 * t77436;
    let t77439 = 0.5255791827870410156e-5_f64 * t75051;
    let t77441 = 0.85129199786595678799e-5_f64 * t75060;
    let t77445 = 0.16263363996404810741e-4_f64 * t75077;
    let t77447 = 0.81300399444200075499e-3_f64 * t75084;
    let t77450 = 0.36366215538993788973e-1_f64 * t75088;
    let t77452 = 0.11634323970834742769e-4_f64 * t75096;
    (t77428, t77430, t77431, t77437, t77439, t77441, t77445, t77447, t77450, t77452)
}
