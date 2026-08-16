//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 903/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk903(t13456: f64, t5634: f64, t3484: f64, t5633: f64, t13299: f64, t3796: f64, t3936: f64, t3959: f64, t1163: f64, t3961: f64, t1322: f64, t3579: f64) -> (f64, f64, f64, f64) {
    let t13464 = t5634 * t13456;
    let t13465 = t3484 * t13464;
    let t13466 = t5633 * t13465;
    let t13468 = t5634 * t13299;
    let t13469 = t3796 * t13468;
    let t13470 = t5633 * t13469;
    let t13472 = t3936 * t3959;
    let t13473 = t1163 * t3961;
    let t13474 = t13472 * t13473;
    let t13477 = t3579 * t1322;
    (t13466, t13470, t13474, t13477)
}
