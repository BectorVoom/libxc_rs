//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 959/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk959(t14902: f64, t10243: f64, t10246: f64, t10276: f64, t10282: f64, t10286: f64, t10394: f64, t10398: f64, t14688: f64, t14692: f64, t14697: f64, t14701: f64, t14706: f64, t14708: f64, t14711: f64, t14715: f64, t14718: f64, t14892: f64, t14895: f64, t14899: f64) -> f64 {
    let t14903 = t14902 / 9.0_f64;
    let t14904 = 2.0_f64 / 27.0_f64 * t14688 - 2.0_f64 / 9.0_f64 * t14692 + 2.0_f64 / 3.0_f64 * t14697 + t14701 / 3.0_f64 - t14706 + t10394 / 18.0_f64 - t14708 - t10276 / 9.0_f64 - t10246 / 27.0_f64 - t14711 + t10282 / 54.0_f64 + t10286 / 81.0_f64 - 2.0_f64 / 81.0_f64 * t14715 - 11.0_f64 / 27.0_f64 * t14718 - t10243 / 27.0_f64 - t14892 / 6.0_f64 - 2.0_f64 / 27.0_f64 * t14895 + t14899 / 9.0_f64 + t14903 - t10398;
    t14904
}
