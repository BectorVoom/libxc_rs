//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2151/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2151(t3061: f64, t676: f64, t1041: f64, t248: f64, t2771: f64, t3129: f64, t42742: f64, t10962: f64, t3103: f64, t3078: f64, t3082: f64, t3089: f64) -> (f64, f64, f64, f64, f64) {
    let t43338 = t676 * t3061;
    let t43341 = t1041 * t248 * t43338 * t2771;
    let t43343 = t42742 * t3129;
    let t43350 = t10962 * t3103;
    let t43352 = t3078 * t3082;
    let t43354 = t3089 * t3082;
    (t43341, t43343, t43350, t43352, t43354)
}
