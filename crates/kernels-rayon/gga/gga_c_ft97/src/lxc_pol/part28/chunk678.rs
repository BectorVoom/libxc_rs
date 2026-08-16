//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 678/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk678(t26849: f64, t574: f64, t605: f64, t23443: f64, t3446: f64, t23470: f64, t3430: f64, t3435: f64, t1378: f64, t2097: f64, t3441: f64, t1901: f64, t23425: f64, t23427: f64, t26826: f64, t26830: f64, t26833: f64, t26838: f64, t26842: f64, t26846: f64, t446: f64) -> f64 {
    let t26851 = t574 * t605 * t26849;
    let t26854 = t23443 * t3446;
    let t26857 = t23470 * t3430;
    let t26860 = t23470 * t3435;
    let t26863 = t2097 * t1378;
    let t26864 = t26863 * t3441;
    let t26867 = -2.0_f64 / 9.0_f64 * t26826 - t23425 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t23427 + 2.0_f64 / 3.0_f64 * t446 * t26830 + 2.0_f64 / 3.0_f64 * t446 * t26833 + t446 * t26838 / 3.0_f64 + t446 * t26842 / 3.0_f64 + t446 * t26846 / 3.0_f64 + t446 * t26851 / 3.0_f64 + t1901 * t26854 / 9.0_f64 + t1901 * t26857 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t26860 - 2.0_f64 / 27.0_f64 * t1901 * t26864;
    t26867
}
