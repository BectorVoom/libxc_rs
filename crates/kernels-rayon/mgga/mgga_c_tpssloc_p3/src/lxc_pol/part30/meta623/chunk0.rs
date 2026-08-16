//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2023/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2023(t86942: f64, t23168: f64, t25338: f64, t23012: f64, t7485: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t225: f64, t25051: f64, t7489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86943 = 0.38381794893125283518e-1_f64 * t86942;
    let t86950 = t23168 * t25338;
    let t86951 = 0.76763589786250567036e-1_f64 * t86950;
    let t86955 = t23012 * t7485;
    let t86967 = t6579 * t25046;
    let t86968 = 0.76763589786250567036e-1_f64 * t86967;
    let t86969 = t2717 * t1484;
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    (t86943, t86951, t86955, t86968, t86969, t86988, t86991)
}
