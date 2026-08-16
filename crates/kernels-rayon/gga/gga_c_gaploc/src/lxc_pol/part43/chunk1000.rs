//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1000/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1000(t447: f64, t46849: f64, t204: f64, t2476: f64, t40225: f64, t38674: f64, t544: f64, t9287: f64, t2365: f64, t38272: f64, t7025: f64, t38770: f64, t901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47953 = t46849 * t447;
    let t47955 = t2476 * t204 * t47953;
    let t47963 = 0.15337170381568299871e1_f64 * t40225;
    let t47964 = t544 * t38674;
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47976 = t38770 * t901;
    (t47953, t47955, t47963, t47965, t47968, t47976)
}
