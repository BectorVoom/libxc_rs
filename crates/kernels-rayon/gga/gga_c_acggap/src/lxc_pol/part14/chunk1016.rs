//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1016/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1016(t35844: f64, t7447: f64, t8823: f64, t7440: f64, t8826: f64, t30817: f64, t8948: f64, t8793: f64, t1313: f64, t30598: f64, t721: f64, t1322: f64, t7859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35845 = 0.21437009059034868486e-3_f64 * t35844;
    let t35848 = t7447 * t8823;
    let t35849 = 0.84046875e-1_f64 * t35848;
    let t35850 = t7440 * t8826;
    let t35851 = 0.5603125e-1_f64 * t35850;
    let t35874 = t30817 * t8948;
    let t35875 = 0.25724410870841842184e-2_f64 * t35874;
    let t35876 = t30817 * t8793;
    let t35877 = 0.37737710747524982482e-2_f64 * t35876;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    (t35845, t35849, t35851, t35875, t35877, t35882, t35885)
}
