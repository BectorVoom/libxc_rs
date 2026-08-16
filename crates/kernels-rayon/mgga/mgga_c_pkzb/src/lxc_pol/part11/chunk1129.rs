//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1129/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1129(t1542: f64, t3426: f64, t1508: f64, t8770: f64, t114: f64, t557: f64, t8748: f64, t1499: f64, t545: f64, t83: f64, t1532: f64, t3380: f64, t49: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24534 = t1542 * t3426;
    let t24536 = t8770 * t1508;
    let t24539 = t8748 * t114 * t557;
    let t24542 = t8770 * t1499;
    let t24600 = t83 * t8748 * t545;
    let t24604 = t3380 * t49 * t1532;
    (t24534, t24536, t24539, t24542, t24600, t24604)
}
