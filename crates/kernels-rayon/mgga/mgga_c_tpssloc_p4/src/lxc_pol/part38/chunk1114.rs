//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1114/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1114(t10189: f64, t1597: f64, t2990: f64, t2986: f64, t2987: f64, t4540: f64, t10245: f64, t4531: f64, t10241: f64, t4514: f64, t2989: f64, t3966: f64) -> (f64, f64, f64, f64, f64) {
    let t13847 = t10189 * t1597;
    let t13848 = t13847 * t2990;
    let t13850 = 0.18518518518518518518e-3_f64 * t2986 * t13848;
    let t13851 = t2987 * t4540;
    let t13852 = t13851 * t2990;
    let t13855 = t4531 * t10245;
    let t13858 = t10241 * t4514;
    let t13861 = t2989 * t3966;
    (t13850, t13852, t13855, t13858, t13861)
}
