//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2775/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775(t5584: f64, t828: f64, t16946: f64, t2697: f64, t16951: f64, t5614: f64, t9671: f64, t13222: f64, t13223: f64, t13251: f64, t13353: f64, t1512: f64, t16662: f64, t16853: f64, t16859: f64, t2379: f64, t2553: f64, t2618: f64, t2623: f64, t2630: f64, t2643: f64, t2647: f64, t2701: f64, t4234: f64, t46692: f64, t46870: f64, t46874: f64, t47220: f64, t5544: f64, t58281: f64, t58340: f64, t776: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9613: f64) -> f64 {
    let t58688 = t5584 * t828;
    let t58705 = t2697 * t16946;
    let t58709 = t2697 * t16951;
    let t58723 = t9671 * t5614;
    let t58725 = t2630 * t819 * t820 * t58340 / 768.0_f64 - 5.0_f64 / 128.0_f64 * t843 * t9607 * t820 * t5544 * t2379 - 7.0_f64 / 576.0_f64 * t46870 + 119.0_f64 / 1728.0_f64 * t46874 - t2643 * t46692 * t13223 * t4234 / 768.0_f64 + t2643 * t13222 * t58688 * t2647 / 384.0_f64 - 5.0_f64 / 192.0_f64 * t13251 * t13353 - t9613 * t5614 / 3072.0_f64 - t2618 * t16859 / 1536.0_f64 - t817 * t819 * t820 * t58281 / 1536.0_f64 - 5.0_f64 / 64.0_f64 * t2623 * t16853 - 35.0_f64 / 288.0_f64 * t58705 - t47220 * t1512 / 1536.0_f64 - 35.0_f64 / 576.0_f64 * t58709 + 5.0_f64 / 384.0_f64 * t2623 * t16951 + 5.0_f64 / 384.0_f64 * t843 * t2701 * t820 * t16662 * t776 + 5.0_f64 / 768.0_f64 * t843 * t2701 * t820 * t5544 * t2553 - 119.0_f64 / 13824.0_f64 * t58723;
    t58725
}
