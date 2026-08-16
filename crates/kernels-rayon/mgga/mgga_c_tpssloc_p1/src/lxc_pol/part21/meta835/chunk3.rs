//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2965/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965(t13961: f64, t4641: f64, t14137: f64, t4644: f64, t12606: f64, t1409: f64, t10408: f64, t1041: f64, t10891: f64, t13555: f64, t13559: f64, t14077: f64, t1616: f64, t17632: f64, t17962: f64, t3070: f64, t3071: f64, t3109: f64, t42743: f64, t4337: f64, t4582: f64, t4583: f64, t4652: f64, t48460: f64, t48463: f64, t5880: f64, t61768: f64, t61775: f64, t61782: f64, t61784: f64) -> (f64, f64) {
    let t61794 = t4641 * t13961;
    let t61796 = t4644 * t14137;
    let t61798 = t1409 * t12606;
    let t61803 = t3070 * t3071 * t1616 * t13559 / 384.0_f64 - t48460 / 864.0_f64 + 5.0_f64 / 5184.0_f64 * t48463 + 5.0_f64 / 5184.0_f64 * t61768 - 5.0_f64 / 1152.0_f64 * t3070 * t10408 * t1616 * t13555 + 5.0_f64 / 3456.0_f64 * t3070 * t10408 * t4337 * t61775 - t61782 / 20736.0_f64 - t61784 / 864.0_f64 + t10891 * t17632 / 144.0_f64 - t3109 * t17962 / 288.0_f64 - t42743 * t5880 / 3072.0_f64 - t14077 * t4652 / 144.0_f64 + t61794 / 1152.0_f64 + 5.0_f64 / 5184.0_f64 * t61796 - t1041 * t4582 * t4583 * t61798 / 1152.0_f64;
    (t61798, t61803)
}
