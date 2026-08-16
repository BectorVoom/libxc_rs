//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1268/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1268(t10610: f64, t3472: f64, t42432: f64, t11465: f64, t12567: f64, t11325: f64, t12395: f64, t3262: f64, t12945: f64, t37282: f64, t12215: f64, t42945: f64) -> (f64, f64, f64, f64, f64) {
    let t44926 = 15.0_f64 / 8.0_f64 * t10610 * t3472 * t42432;
    let t44928 = 5.0_f64 / 16.0_f64 * t12567 * t11465;
    let t44931 = 15.0_f64 / 8.0_f64 * t3262 * t11325 * t12395;
    let t44933 = 3.0_f64 / 2.0_f64 * t37282 * t12945;
    let t44935 = 3.0_f64 * t42945 * t12215;
    (t44926, t44928, t44931, t44933, t44935)
}
