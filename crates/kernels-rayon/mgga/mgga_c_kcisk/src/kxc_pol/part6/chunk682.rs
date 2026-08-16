//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 682/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk682(t10487: f64, t708: f64, t10671: f64, t677: f64, t10568: f64, t5101: f64, t707: f64, t1797: f64, t180: f64, t479: f64, t574: f64, t682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11328 = t708 * t10487;
    let t11352 = t10671 * t677;
    let t11371 = 0.12841111111111111111e-1_f64 * t10568;
    let t11393 = t707 * t5101;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    (t11328, t11352, t11371, t11393, t11400, t11401)
}
