//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2148/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2148(t25207: f64, t61203: f64, t4433: f64, t605: f64, t892: f64, t14749: f64, t27159: f64, t198: f64, t7188: f64, t11064: f64, t7782: f64, t1468: f64, t1940: f64, t2403: f64, t25206: f64, t25436: f64, t25446: f64, t25452: f64, t27158: f64, t27173: f64, t27368: f64, t27385: f64, t51780: f64, t7087: f64, t7091: f64, t7750: f64, t98684: f64, t98688: f64, t98694: f64, t98699: f64, t98702: f64, t98705: f64) -> (f64, f64, f64) {
    let t98709 = t25207 * t61203;
    let t98713 = t892 * t605 * t4433;
    let t98716 = t27159 * t14749;
    let t98719 = t198 * t7188;
    let t98722 = t7782 * t11064;
    let t98725 = -t1940 * t27368 * t25452 / 2.0_f64 + t98684 + t1940 * t25436 * t1468 / 2.0_f64 + 3.0_f64 * t25206 * t98688 + 3.0_f64 * t2403 * t7087 * t27173 - 3.0_f64 * t25206 * t98694 + 3.0_f64 * t51780 * t7750 + 3.0_f64 * t27158 * t98699 - t1940 * t7091 * t98702 - t1940 * t7091 * t98705 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t25206 * t98709 + 6.0_f64 * t27158 * t98713 + 6.0_f64 * t27158 * t98716 + 2.0_f64 * t98719 * t27385 + t1940 * t98722 * t25446;
    (t98719, t98722, t98725)
}
