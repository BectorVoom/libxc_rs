//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1189/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1189(t11784: f64, t11983: f64, t3784: f64, t3788: f64, t7241: f64, t11990: f64, t19196: f64, t2597: f64, t1086: f64, t11790: f64, t22581: f64, t17760: f64, t2580: f64, t33273: f64) -> (f64, f64, f64, f64, f64) {
    let t33943 = t11784 * t11983;
    let t33946 = t3784 * t7241 * t3788;
    let t33949 = t11990 * t2597 * t19196;
    let t33952 = t11790 * t1086 * t22581;
    let t33956 = t17760 * t33273 * t2580;
    (t33943, t33946, t33949, t33952, t33956)
}
