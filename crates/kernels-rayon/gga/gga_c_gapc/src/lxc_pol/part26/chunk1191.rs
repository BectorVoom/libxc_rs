//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1191/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1191(t1393: f64, t3663: f64, t9229: f64, t11424: f64, t563: f64, t2983: f64, t1787: f64, t3684: f64, t11381: f64, t8787: f64, t11463: f64, t9330: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34752 = t1393 * t3663 * t9229;
    let t34754 = t563 * t11424;
    let t34755 = t34754 * t2983;
    let t34757 = t3684 * t1787;
    let t34759 = t8787 * t11381;
    let t34761 = t11463 * t9330;
    (t34752, t34754, t34755, t34757, t34759, t34761)
}
