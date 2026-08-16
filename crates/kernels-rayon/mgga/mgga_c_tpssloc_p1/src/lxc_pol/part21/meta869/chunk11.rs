//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3193/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193(t18363: f64, t3577: f64, t45124: f64, t11697: f64, t18359: f64, t15459: f64, t15463: f64, t15478: f64, t15631: f64, t15740: f64, t18321: f64, t18368: f64, t3562: f64, t45044: f64, t45049: f64, t45162: f64, t53135: f64, t53142: f64, t53155: f64, t53158: f64, t53161: f64, t53185: f64, t53472: f64) -> f64 {
    let t66334 = t3577 * t45124 * t18363;
    let t66337 = t3577 * t11697 * t18359;
    let t66353 = -t15740 * t15478 / 1152.0_f64 - t15740 * t15459 / 2304.0_f64 - t15740 * t15463 / 1152.0_f64 + 5.0_f64 / 10368.0_f64 * t66334 - t66337 / 1728.0_f64 - 5.0_f64 / 1944.0_f64 * t45044 + t53135 / 1728.0_f64 - 5.0_f64 / 62208.0_f64 * t45049 - t53472 * t15631 / 256.0_f64 + 11.0_f64 / 243.0_f64 * t18321 * t3562 - t53142 / 432.0_f64 - t45162 * t18368 / 1152.0_f64 - t53155 / 3456.0_f64 - t53158 / 1728.0_f64 + t53161 / 5184.0_f64 + t53185 / 2304.0_f64;
    t66353
}
