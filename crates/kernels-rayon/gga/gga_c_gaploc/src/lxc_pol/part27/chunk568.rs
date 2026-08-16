//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 568/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk568(t1457: f64, t2959: f64, t2949: f64, t723: f64, t1445: f64, t2936: f64, t313: f64) -> (f64, f64, f64, f64, f64) {
    let t3015 = t1457 * t2959;
    let t3018 = t2949 * t723;
    let t3019 = t1445 * t3018;
    let t3022 = t1445 * t2959;
    let t3025 = t313 * t2936;
    (t3015, t3018, t3019, t3022, t3025)
}
