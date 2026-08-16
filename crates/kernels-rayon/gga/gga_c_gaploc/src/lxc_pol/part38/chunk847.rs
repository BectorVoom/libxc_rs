//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 847/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk847(t1063: f64, t11264: f64, t6755: f64, t2268: f64, t35045: f64, t7937: f64, t42827: f64, t11232: f64, t894: f64, t2440: f64, t3531: f64, t3338: f64, t7995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44549 = 0.34146007962811379518e0_f64 * t1063 * t11264 * t6755;
    let t44552 = 0.68292015925622759036e0_f64 * t2268 * t7937 * t35045;
    let t44553 = 0.47425011059460249332e-2_f64 * t42827;
    let t44556 = 0.28455006635676149599e-1_f64 * t2268 * t894 * t11232;
    let t44559 = 0.28455006635676149599e-1_f64 * t2268 * t2440 * t3531;
    let t44560 = t7995 * t3338;
    (t44549, t44552, t44553, t44556, t44559, t44560)
}
