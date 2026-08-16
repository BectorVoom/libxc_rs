//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2150/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150(t23097: f64, t4234: f64, t776: f64, t815: f64, t81877: f64, t81883: f64, t13176: f64, t6620: f64, t849: f64, t81857: f64, t81859: f64, t81874: f64, t87287: f64, t87289: f64, t87292: f64, t87293: f64, t87296: f64, t87298: f64, t87301: f64, t87304: f64, t87306: f64, t87308: f64, t87312: f64) -> f64 {
    let t87316 = t23097 * t815 * t4234 * t776;
    let t87319 = 0.33643963411783659044e-4_f64 * t81877;
    let t87320 = 0.10541775202358879834e-2_f64 * t81883;
    let t87321 = t13176 * t6620;
    let t87322 = t87321 * t849;
    let t87324 = -5.0_f64 / 192.0_f64 * t87287 + t87289 / 192.0_f64 + t87292 + 0.16956557559538964158e-1_f64 * t87293 - t87296 / 768.0_f64 - t87298 / 1536.0_f64 - t87301 - 35.0_f64 / 576.0_f64 * t81857 + 0.14130464632949136799e-2_f64 * t81859 - 35.0_f64 / 216.0_f64 * t87304 - 0.67826230238155856634e-1_f64 * t87306 - 0.16956557559538964158e-1_f64 * t87308 + 0.40372756094140390854e-3_f64 * t87312 + 0.24223653656484234512e-2_f64 * t87316 + 0.33643963411783659045e-4_f64 * t81874 + t87319 - t87320 - t87322 / 192.0_f64;
    t87324
}
