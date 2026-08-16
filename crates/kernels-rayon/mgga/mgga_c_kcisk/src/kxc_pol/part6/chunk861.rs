//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 861/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk861(t28377: f64, t4726: f64, t26: f64, t6777: f64, t8522: f64, t2372: f64, t8504: f64, t10663: f64, t10671: f64, t10621: f64, t28369: f64, t1659: f64, t28389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28409 = t4726 * t28377;
    let t28410 = t26 * t28409;
    let t28412 = t6777 * t8522;
    let t28414 = t8504 * t2372;
    let t28415 = t10663 * t28414;
    let t28417 = t10671 * t28414;
    let t28419 = t10621 * t28369;
    let t28420 = t26 * t28419;
    let t28422 = t1659 * t28389;
    (t28410, t28412, t28414, t28415, t28417, t28420, t28422)
}
