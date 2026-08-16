//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1333/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1333(t11698: f64, t24095: f64, t1062: f64, t3728: f64, t6773: f64, t11669: f64, t2456: f64, t11629: f64, t11637: f64, t1061: f64, t23523: f64, t6927: f64) -> (f64, f64, f64, f64, f64) {
    let t35956 = t24095 * t11698;
    let t35959 = t1062 * t3728 * t6773;
    let t35962 = t1062 * t11669 * t2456;
    let t35966 = t11637 * t11629;
    let t35970 = t1061 * t23523 * t3728 * t6927;
    (t35956, t35959, t35962, t35966, t35970)
}
