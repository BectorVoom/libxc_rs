//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 838/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk838<F: Float>(t1035: F, t1039: F, t3669: F, t3036: F, t3213: F, t996: F, t117: F, t3033: F, t111: F, t150: F, t864: F, t174: F, t383: F, t386: F, t387: F, t980: F) -> (F, F, F, F, F, F, F) {
    let t13474 = 0.68026775414003982664e-1 * t1035 * t3669 * t1039;
    let t13481 = 0.24009450146119052705e-1 * t3036 * t996 * t3213;
    let t13483 = 1.0 / t3033 / t117;
    let t13484 = t111 * t13483;
    let t13485 = t13484 * t150;
    let t13487 = t864 * t864;
    let t13492 = 0.51448821741683684368e-2 * t13485 * t383 * t386 * t387 * t174 * t13487;
    let t13502 = t980 * t996;
    (t13474, t13481, t13484, t13485, t13487, t13492, t13502)
}
