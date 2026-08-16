//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1189/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1189(t1184: f64, t6728: f64, t10753: f64, t6720: f64, t14875: f64, t1801: f64, t13321: f64, t3436: f64, t5177: f64, t19540: f64, t355: f64, t381: f64) -> (f64, f64, f64, f64, f64) {
    let t19895 = t1184 * t6728;
    let t19897 = t10753 * t6720;
    let t19899 = t14875 * t1801;
    let t19901 = t13321 * t3436;
    let t19902 = t19901 * t5177;
    let t19904 = t19540 * t355;
    let t19905 = t19904 * t381;
    (t19895, t19897, t19899, t19902, t19905)
}
