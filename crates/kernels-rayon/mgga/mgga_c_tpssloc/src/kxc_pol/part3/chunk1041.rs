//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1041/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1041(t13520: f64, t2845: f64, t10650: f64, t1557: f64, t2787: f64, t4396: f64, t2770: f64, t3966: f64, t607: f64, t2826: f64, t136: f64, t2250: f64, t4337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13522 = 0.16081979498692535067e2_f64 * t13520 * t2845;
    let t13524 = 1.0_f64 * t10650 * t1557;
    let t13526 = 2.0_f64 * t2787 * t4396;
    let t13527 = t2770 * t3966;
    let t13528 = t13527 * t607;
    let t13529 = t2826 * t13528;
    let t13530 = t136 * t13529;
    let t13532 = t4337 * t2250;
    (t13522, t13524, t13526, t13528, t13530, t13532)
}
