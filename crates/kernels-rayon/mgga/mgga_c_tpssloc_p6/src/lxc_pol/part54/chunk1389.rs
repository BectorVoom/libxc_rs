//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1389/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1389(t121593: f64, t121606: f64, t121495: f64, t25038: f64, t25248: f64, t776: f64, t114649: f64, t114752: f64, t118756: f64, t118760: f64, t118764: f64, t118767: f64, t121553: f64, t121560: f64, t121563: f64, t121574: f64, t1510: f64, t226: f64, t235: f64, t2617: f64, t31394: f64, t31395: f64, t33388: f64, t4162: f64, t4166: f64, t4182: f64, t4234: f64, t4281: f64, t4291: f64, t812: f64, t829: f64, t8560: f64) -> (f64, f64) {
    let t121607 = t121593 + t121606;
    let t121612 = t25038 * t25248 * t121495 * t776;
    let t121614 = 2.0_f64 * t4281 * t121553 * t4182 + t4162 * t8560 - t118756 - 0.82246703342411321825e-2_f64 * t121560 - 0.82246703342411321825e-2_f64 * t121563 - t812 * t31394 * t4234 - t2617 * t33388 - t812 * t114649 * t1510 + 0.19190897446562641759e-1_f64 * t114752 - t4166 * t31395 - t118760 - t118764 + t118767 - t4291 * t121553 * t829 - 0.19190897446562641759e-1_f64 * t121574 + t226 * t235 * t121607 + 0.49348022005446793095e-1_f64 * t121612;
    (t121607, t121614)
}
