//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1295/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1295(t120092: f64, t120095: f64, t120097: f64, t120103: f64, t120107: f64, t120111: f64, t123178: f64, t123180: f64, t123182: f64, t123184: f64, t123187: f64, t123189: f64, t123193: f64, t1442: f64, t2165: f64, t27293: f64, t27371: f64, t32572: f64, t32605: f64, t34372: f64, t4028: f64, t652: f64, t671: f64, t7264: f64, t7266: f64, t8103: f64) -> f64 {
    let t125939 = -2.0_f64 * t34372 * t652 * t671 - t1442 * t32572 - 2.0_f64 * t2165 * t27371 - 4.0_f64 * t27293 * t7266 - 2.0_f64 * t32605 * t4028 - 2.0_f64 * t7264 * t8103 - t120092 + t120095 - t120097 + t120103 + t120107 - t120111 - 6.0_f64 * t123178 - 4.0_f64 * t123180 - 4.0_f64 * t123182 - 4.0_f64 * t123184 - 4.0_f64 * t123187 + 2.0_f64 * t123189 + 2.0_f64 * t123193;
    t125939
}
