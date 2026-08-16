//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2298/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2298(t1011: f64, t5011: f64, t11715: f64, t491: f64, t85964: f64, t27488: f64, t1209: f64, t1216: f64, t1235: f64, t15018: f64, t15620: f64, t15625: f64, t24762: f64, t24812: f64, t24813: f64, t24814: f64, t24815: f64, t24833: f64, t24834: f64, t27406: f64, t27470: f64, t27471: f64, t27489: f64, t27490: f64, t27496: f64, t27497: f64, t27501: f64, t27507: f64, t3494: f64, t3509: f64, t3604: f64, t3610: f64, t5068: f64, t7373: f64, t85963: f64, t94875: f64) -> (f64, f64) {
    let t94986 = t5011 * t1011;
    let t95000 = t85964 * t11715 * t491;
    let t95005 = t85964 * t27488;
    let t95026 = -0.16449340668482264365e-1_f64 * t24812 * t24813 * t1209 * t1235 * t27497 - 0.16449340668482264365e-1_f64 * t24812 * t27496 * t94986 * t1216 - 0.82246703342411321825e-2_f64 * t24812 * t27496 * t27490 * t3494 + 0.16449340668482264365e-1_f64 * t24812 * t27489 * t27490 * t15620 + 0.49348022005446793095e-1_f64 * t85963 * t95000 * t94875 * t15625 - 0.49348022005446793095e-1_f64 * t85963 * t95005 * t94875 * t3509 + 0.43864908449286038306e-1_f64 * t27507 * t24834 - 0.16449340668482264365e-1_f64 * t7373 * t24833 * t27501 + 0.21932454224643019153e-1_f64 * t27406 * t24762 + 2.0_f64 * t3604 * t27471 + 4.0_f64 * t3610 * t27470 * t5068 + 0.16449340668482264365e-1_f64 * t24812 * t24814 * t15018 * t24815;
    (t94986, t95026)
}
