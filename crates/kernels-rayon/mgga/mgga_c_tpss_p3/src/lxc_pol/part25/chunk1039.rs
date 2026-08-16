//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1039/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1039(t10572: f64, t1379: f64, t3683: f64, t10578: f64, t10579: f64, t4722: f64, t4707: f64, t750: f64, t762: f64, t1368: f64, t3610: f64, t4711: f64) -> (f64, f64, f64, f64, f64) {
    let t14322 = t10572 * t1379 * t3683;
    let t14326 = t10578 * t10579 * t4722;
    let t14330 = t762 * t4707 * t750;
    let t14334 = t762 * t1368 * t3610;
    let t14338 = t762 * t4711 * t750;
    (t14322, t14326, t14330, t14334, t14338)
}
