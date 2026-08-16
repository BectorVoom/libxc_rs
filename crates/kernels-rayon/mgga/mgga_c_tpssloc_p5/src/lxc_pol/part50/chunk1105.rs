//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1105/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1105(t1055: f64, t32964: f64, t1539: f64, t30781: f64, t23329: f64, t1599: f64, t30861: f64, t25406: f64, t8380: f64, t23394: f64, t7599: f64, t6704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32965 = t1055 * t32964;
    let t32969 = t30781 * t1539;
    let t32970 = t23329 * t32969;
    let t32973 = t1599 * t30861;
    let t32976 = t25406 * t8380;
    let t32980 = t23394 * t7599;
    let t32981 = t6704 * t32980;
    (t32965, t32969, t32970, t32973, t32976, t32980, t32981)
}
