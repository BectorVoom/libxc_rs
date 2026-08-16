//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1075/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1075(t1241: f64, t32479: f64, t225: f64, t8883: f64, t1251: f64, t8897: f64, t3598: f64, t2154: f64, t7391: f64, t1170: f64, t8867: f64, t2121: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32480 = t1241 * t32479;
    let t32482 = t8883 * t225;
    let t32488 = t8897 * t1251;
    let t32489 = t3598 * t32488;
    let t32492 = t2154 * t7391;
    let t32493 = t3598 * t32492;
    let t32496 = t1170 * t8867;
    let t32498 = 0.54831135561607547883e-2_f64 * t2121 * t32496;
    (t32480, t32482, t32489, t32493, t32496, t32498)
}
