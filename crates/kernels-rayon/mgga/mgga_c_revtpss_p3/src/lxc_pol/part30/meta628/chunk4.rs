//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2190/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2190(t25759: f64, t61182: f64, t101029: f64, t101032: f64, t101035: f64, t101040: f64, t101051: f64, t101055: f64, t1711: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25436: f64, t25445: f64, t25763: f64, t25778: f64, t27158: f64, t27773: f64, t27800: f64, t7087: f64, t7207: f64, t7783: f64, t7862: f64, t98719: f64, t98722: f64, t98784: f64, t99555: f64) -> f64 {
    let t101061 = t25759 * t61182;
    let t101064 = 2.0_f64 * t98719 * t27800 + 3.0_f64 * t2403 * t7783 * t25763 + 6.0_f64 * t27158 * t101029 + 6.0_f64 * t27158 * t101032 + 3.0_f64 * t27158 * t101035 - t1940 * t99555 * t7207 + t1940 * t25445 * t101040 + t1940 * t98722 * t25778 + t1940 * t25436 * t1711 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t25436 * t7862 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t101051 + t98784 - 3.0_f64 * t25206 * t101055 + 3.0_f64 * t2403 * t7087 * t27773 - 3.0_f64 * t25206 * t101061;
    t101064
}
