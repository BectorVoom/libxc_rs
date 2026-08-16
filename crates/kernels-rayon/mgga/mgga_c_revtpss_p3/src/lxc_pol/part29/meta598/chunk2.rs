//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2029/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2029(t103553: f64, t892: f64, t26425: f64, t98648: f64, t1940: f64, t2255: f64, t7428: f64, t102917: f64, t2071: f64, t2403: f64, t25215: f64, t26585: f64, t27173: f64, t27387: f64, t28291: f64, t28472: f64, t30: f64, t4541: f64, t7432: f64, t8020: f64, t98652: f64, t98675: f64, t98705: f64, t98709: f64, t98736: f64, t98780: f64, t98793: f64, t99543: f64) -> (f64, f64, f64, f64) {
    let t103554 = t103553 * t892;
    let t103561 = 6.0_f64 * t26425 * t98648;
    let t103570 = 2.0_f64 * t1940 * t7428 * t2255;
    let t103574 = 3.0_f64 * t2403 * t7428 * t27173 + 3.0_f64 * t4541 * t2071 * t98793 - 3.0_f64 / 2.0_f64 * t26425 * t98652 - t102917 - 6.0_f64 * t28291 * t98675 + t28472 * t98780 - t1940 * t7432 * t98736 / 2.0_f64 - t1940 * t26585 * t27387 + t1940 * t103554 * t30 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t98709 + t103561 - t1940 * t7432 * t98705 / 2.0_f64 + 3.0_f64 * t2403 * t2071 * t99543 + t103570 + 3.0_f64 / 2.0_f64 * t2403 * t8020 * t25215;
    (t103554, t103561, t103570, t103574)
}
