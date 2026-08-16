//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2285/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2285(t1011: f64, t1212: f64, t65955: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t15495: f64, t4997: f64, t15492: f64, t5019: f64, t15591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65957 = t65955 * t1011 * t1212;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    let t65966 = t65962 * t3514;
    let t65992 = t15495 * t4997;
    let t65994 = t5019 * t15492;
    let t65996 = t15591 * t4997;
    (t65957, t65963, t65966, t65992, t65994, t65996)
}
