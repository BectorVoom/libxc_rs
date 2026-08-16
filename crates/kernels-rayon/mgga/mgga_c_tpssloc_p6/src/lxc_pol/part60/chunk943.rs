//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 943/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk943(t114172: f64, t22892: f64, t7691: f64, t3886: f64, t7749: f64, t31169: f64, t5234: f64, t114011: f64, t32721: f64, t1824: f64, t22705: f64, t22852: f64, t550: f64, t59: f64) -> (f64, f64, f64, f64, f64) {
    let t120308 = t22892 * t114172 * t7691;
    let t120317 = t3886 * t7749;
    let t120341 = t5234 * t31169;
    let t120350 = t114011 * t32721;
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    (t120308, t120317, t120341, t120350, t120363)
}
