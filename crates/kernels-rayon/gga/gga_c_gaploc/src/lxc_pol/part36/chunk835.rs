//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 835/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk835(t12766: f64, t4614: f64, t597: f64, t12905: f64, t1641: f64, t1445: f64, t31501: f64, t574: f64, t874: f64, t40147: f64, t12792: f64, t158: f64) -> (f64, f64, f64, f64, f64) {
    let t41793 = 0.15337170381568299871e2_f64 * t597 * t4614 * t12766;
    let t41794 = t1641 * t12905;
    let t41798 = t574 * t1445 * t31501 * t874;
    let t41800 = 0.11502877786176224903e1_f64 * t40147;
    let t41801 = t158 * t12792;
    (t41793, t41794, t41798, t41800, t41801)
}
