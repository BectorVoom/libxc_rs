//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 862/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk862(t26: f64, t28422: f64, t1659: f64, t28381: f64, t28373: f64, t4726: f64, t1653: f64, t28393: f64, t10739: f64, t28371: f64, t28375: f64, t28383: f64, t28391: f64, t28410: f64, t28412: f64, t28415: f64, t28417: f64, t28420: f64) -> (f64, f64, f64, f64, f64) {
    let t28423 = t26 * t28422;
    let t28425 = t1659 * t28381;
    let t28426 = t26 * t28425;
    let t28430 = t4726 * t28373;
    let t28431 = t26 * t28430;
    let t28435 = t1653 * t28393;
    let t28437 = -0.82156666666666666668e-1_f64 * t28410 - 0.28483875e1_f64 * t28412 - t10739 - 0.76790625e-1_f64 * t28415 + 0.142419375e1_f64 * t28417 - 0.36514074074074074075e-1_f64 * t28420 - 0.82156666666666666667e-1_f64 * t28423 - 0.49293999999999999999e0_f64 * t28426 + 0.11958666666666666667e1_f64 * t28375 - 0.17938e1_f64 * t28383 + 0.16431333333333333333e0_f64 * t28431 - 0.33218518518518518518e0_f64 * t28371 - 0.29896666666666666667e0_f64 * t28391 + 0.3071625e0_f64 * t28435;
    (t28423, t28426, t28431, t28435, t28437)
}
