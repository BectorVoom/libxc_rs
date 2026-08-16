//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2342/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342(t26012: f64, t7255: f64, t22527: f64, t22549: f64, t24514: f64, t24517: f64, t24520: f64, t26009: f64, t26090: f64, t27298: f64, t27303: f64, t27332: f64, t6495: f64, t83722: f64, t83778: f64, t85463: f64, t85480: f64, t85501: f64, t85536: f64, t90080: f64, t90114: f64, t90137: f64, t90141: f64) -> f64 {
    let t96102 = t7255 * t26012;
    let t96105 = 5.0_f64 / 3.0_f64 * t27332 * t22527 + 2.0_f64 / 3.0_f64 * t6495 * t27303 + 5.0_f64 / 3.0_f64 * t24520 * t26090 - 10.0_f64 / 3.0_f64 * t90114 * t24517 + 10.0_f64 * t90137 * t85463 + 35.0_f64 * t85501 * t90141 - 10.0_f64 * t85536 * t26009 - 10.0_f64 * t85480 * t26009 - 5.0_f64 * t24514 * t90080 - 10.0_f64 / 3.0_f64 * t83722 * t27298 - 5.0_f64 / 3.0_f64 * t83778 * t27298 - 10.0_f64 / 3.0_f64 * t22549 * t96102;
    t96105
}
