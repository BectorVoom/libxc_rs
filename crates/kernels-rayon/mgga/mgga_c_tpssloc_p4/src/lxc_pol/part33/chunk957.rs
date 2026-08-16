//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 957/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk957(t20857: f64, t819: f64, t820: f64, t20800: f64, t847: f64, t20756: f64, t210: f64, t214: f64, t221: f64, t4128: f64, t5544: f64, t12986: f64, t13010: f64, t13022: f64, t16769: f64, t16784: f64, t16792: f64, t16794: f64, t4127: f64, t787: f64, t9540: f64, t9559: f64, t9572: f64, t9579: f64, t9583: f64) -> (f64, f64, f64) {
    let t20904 = t819 * t820 * t20857;
    let t20908 = t847 * t820 * t20800;
    let t20923 = t210 * t214 * t20756;
    let t20927 = t221 * t4128 * t5544;
    let t20933 = t210 * t214 * t20800;
    let t20936 = -t9540 + 0.49999999999999999998e-2_f64 * t12986 - t9572 - 0.34999999999999999998e-1_f64 * t16769 - 0.38888888888888888888e-1_f64 * t13010 - 0.74999999999999999997e-2_f64 * t16784 + 0.24999999999999999999e-2_f64 * t16792 - 0.19999999999999999999e-1_f64 * t9559 * t20923 + 0.14999999999999999999e-1_f64 * t4127 * t20927 + t9579 + 0.11666666666666666666e-1_f64 * t16794 - 0.15833333333333333333e-1_f64 * t13022 - 0.16666666666666666666e-2_f64 * t787 * t20933 - t9583;
    (t20904, t20908, t20936)
}
