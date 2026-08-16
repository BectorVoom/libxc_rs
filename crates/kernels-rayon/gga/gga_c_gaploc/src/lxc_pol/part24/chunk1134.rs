//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1134/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1134(t3130: f64, t6338: f64, t1217: f64, t29874: f64, t9208: f64, t4325: f64, t6515: f64, t6525: f64, t484: f64, t9090: f64, t20395: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30182 = 0.23712505529730124666e-2_f64 * t6338 * t3130;
    let t30184 = 0.73772239425827054516e-2_f64 * t1217 * t3130;
    let t30186 = 0.94850022118920498664e-2_f64 * t29874 * t9208;
    let t30189 = 0.142275033178380748e-1_f64 * t6525 * t6515 * t4325;
    let t30199 = 0.63233348079280332442e-2_f64 * t484 * t9090;
    let t30200 = t493 * t20395;
    (t30182, t30184, t30186, t30189, t30199, t30200)
}
