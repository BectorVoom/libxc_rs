//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1009/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1009(t32545: f64, t3255: f64, t26113: f64, t5710: f64, t1286: f64, t34365: f64, t376: f64, t1586: f64, t34482: f64, t136121: f64, t137262: f64, t137525: f64, t25605: f64, t28: f64, t2976: f64, t3109: f64, t32380: f64, t34577: f64, t34585: f64, t5495: f64, t5501: f64, t5508: f64, t5618: f64, t6414: f64, t6562: f64, t7286: f64) -> (f64, f64, f64, f64) {
    let t144648 = t32545 * t3255;
    let t144657 = t5710 * t26113;
    let t144664 = t1286 * t376 * t34365;
    let t144666 = t1586 * t34482;
    let t144676 = -2.0_f64 * t144648 - t136121 / 27.0_f64 + t6414 * t32380 / 6.0_f64 + t137262 + t1286 * t28 * t5618 * t6562 / 3.0_f64 - 4.0_f64 * t144657 - t2976 * t7286 - t3109 * t7286 + t5495 * t34577 / 6.0_f64 - t144664 / 3.0_f64 - t1286 * t28 * t144666 * t5508 / 3.0_f64 + t5501 * t137525 * t25605 / 9.0_f64 + t5495 * t34585 / 3.0_f64;
    (t144648, t144657, t144666, t144676)
}
