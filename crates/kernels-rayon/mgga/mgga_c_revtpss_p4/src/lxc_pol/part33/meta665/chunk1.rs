//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2175/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175(t1444: f64, t6874: f64, t22453: f64, t94901: f64, t108368: f64, t25895: f64, t108225: f64, t14230: f64, t25930: f64, t25931: f64, t27868: f64, t27973: f64, t27981: f64, t3999: f64, t6918: f64, t7274: f64, t7295: f64, t7296: f64, t75012: f64, t7910: f64, t94865: f64, t94867: f64, t97933: f64, t98084: f64, t98089: f64, t98091: f64, t98099: f64) -> f64 {
    let t108448 = t6874 * t1444;
    let t108455 = t94901 * t22453;
    let t108464 = t25895 * t108368;
    let t108471 = -t94865 - 0.17347256376410398924e1_f64 * t108225 * t27981 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t108448 + 0.8673628188205199462e0_f64 * t27868 * t25931 * t75012 + 0.19514881078765566037e-1_f64 * t108455 - t94867 - 0.45699670022203476294e-2_f64 * t98084 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t7274 * t6918 - 0.17347256376410398924e1_f64 * t97933 * t27973 + t98089 - t98091 - 0.14456046980341999104e-1_f64 * t108464 - 0.17347256376410398924e1_f64 * t27868 * t3999 * t7910 * t14230 - 0.4818682326780666368e-3_f64 * t98099;
    t108471
}
