//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2199/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2199(t30031: f64, t686: f64, t72: f64, t25878: f64, t1955: f64, t22307: f64, t1903: f64, t2030: f64, t26084: f64, t27960: f64, t5774: f64, t6896: f64, t7295: f64, t7296: f64, t7910: f64, t94735: f64, t94756: f64, t94758: f64, t97951: f64, t97953: f64, t97956: f64, t97964: f64, t97968: f64, t97974: f64) -> (f64, f64) {
    let t108368 = t30031 * t72 * t686;
    let t108369 = t25878 * t108368;
    let t108371 = t1955 * t22307;
    let t108374 = t97951 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t7910 * t5774 - t97953 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t27960 * t1903 + 0.13170898365871023197e1_f64 * t26084 * t6896 - 0.13009920719177044025e-1_f64 * t94735 + 0.4818682326780666368e-3_f64 * t97956 - 0.96373646535613327357e-2_f64 * t94756 - t97964 + 0.73171657588172351096e-2_f64 * t94758 + 0.25702851531048074406e-1_f64 * t108369 - t97968 + t97974 - 0.4336814094102599731e0_f64 * t108371 * t2030;
    (t108368, t108374)
}
