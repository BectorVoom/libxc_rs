//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 229/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk229(t772: f64, t876: f64, t284: f64, t316: f64, t344: f64, t366: f64, t724: f64, t727: f64, t730: f64, t731: f64, t763: f64, t771: f64, t788: f64, t794: f64, t795: f64, t799: f64, t802: f64, t821: f64, t828: f64, t832: f64, t835: f64) -> (f64, f64) {
    let t877 = t772 * t876;
    let t880 = t344 + t366 + t724 - t727 - t730 - 0.46971924784082831588e-3_f64 * t731 * t316 + 0.28183154870449698953e-3_f64 * t763 * t316 - 0.28183154870449698953e-3_f64 * t771 * t788 - 0.93943849568165663176e-5_f64 * t794 * t795 + 0.16703216453219854913e-4_f64 * t799 * t802 + 0.28183154870449698953e-3_f64 * t284 * t821 + 0.1370014472869082588e-4_f64 * t828 * t832 - 0.28183154870449698953e-3_f64 * t835 * t877;
    (t877, t880)
}
