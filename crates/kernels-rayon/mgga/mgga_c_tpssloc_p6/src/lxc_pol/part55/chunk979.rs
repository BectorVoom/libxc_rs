//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 979/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk979(t26446: f64, t26447: f64, t26331: f64, t26403: f64, t5250: f64, t5287: f64, t6987: f64, t1338: f64, t7722: f64, t1352: f64, t16036: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t26448 = t26446 * t26447;
    let t26449 = t26331 * t26448;
    let t26453 = t26403 * t5250;
    let t26456 = t6987 * t5287;
    let t26458 = t1338 * t7722;
    let t26459 = t26458 * t1352;
    let t26461 = t16036 * t550;
    (t26449, t26453, t26456, t26459, t26461)
}
