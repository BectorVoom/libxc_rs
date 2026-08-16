//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1335/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1335(t31169: f64, t5234: f64, t31172: f64, t114002: f64, t32721: f64, t16242: f64, t31170: f64, t5248: f64, t550: f64, t114011: f64, t12419: f64, t1307: f64, t1336: f64, t240: f64, t241: f64, t5301: f64, t552: f64) -> (f64, f64, f64, f64, f64) {
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    let t120344 = t114002 * t32721;
    let t120348 = t31170 * t5248 * t16242 * t550;
    let t120350 = t114011 * t32721;
    let t120357 = t1336 * t552 * t240 * t241 * t12419 * t5301 * t1307;
    (t120342, t120344, t120348, t120350, t120357)
}
