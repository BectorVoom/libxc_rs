//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 730/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk730(t23103: f64, t23104: f64, t1899: f64, t2693: f64, t281: f64, t6598: f64, t22690: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23105 = t23103 * t23104;
    let t23106 = 0.16821981705891829522e-4_f64 * t23105;
    let t23107 = t1899 * t2693;
    let t23108 = 119.0_f64 / 6912.0_f64 * t23107;
    let t23109 = t6598 * t281;
    let t23110 = t22690 * t814;
    (t23105, t23106, t23107, t23108, t23109, t23110)
}
