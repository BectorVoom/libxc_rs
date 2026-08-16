//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1207/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1207(t4729: f64, t4908: f64, t1178: f64, t3966: f64, t1177: f64, t135: f64, t1716: f64, t1174: f64, t1714: f64, t3448: f64) -> (f64, f64, f64, f64, f64) {
    let t4909 = t4908 * t4729;
    let t4912 = t1178 * t3966;
    let t4913 = t1177 * t4912;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    (t4909, t4912, t4913, t4917, t4919)
}
