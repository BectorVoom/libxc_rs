//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1080/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1080(t265: f64, t504: f64, t32507: f64, t32550: f64, t3640: f64, t8900: f64, t11947: f64, t8904: f64, t1254: f64, t1256: f64, t193: f64, t30952: f64, t336: f64, t4700: f64, t7394: f64, t7398: f64) -> (f64, f64, f64, f64) {
    let t505 = t265 < t504;
    let t32551 = t32507 + t32550;
    let t32555 = t8900 * t3640;
    let t32561 = t8904 * t11947;
    let t32566 = piecewise3(t505, t1256 * t193 * t32551 * t336 - t1254 * t32555 * t4700 + 2.0_f64 * t1254 * t32561 * t4700 - 2.0_f64 * t4700 * t7394 * t7398, t30952);
    (t32551, t32555, t32561, t32566)
}
