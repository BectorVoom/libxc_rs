//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1299/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1299(t30465: f64, t580: f64, t1858: f64, t8240: f64, t30500: f64, t576: f64, t2186: f64, t671: f64, t110631: f64, t110684: f64, t111594: f64, t12524: f64, t1458: f64, t16524: f64, t19534: f64, t20176: f64, t28893: f64, t29993: f64, t29996: f64, t30180: f64, t30253: f64, t30258: f64, t30424: f64, t30492: f64, t30495: f64, t33185: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t5456: f64, t5493: f64, t577: f64, t75795: f64, t8143: f64, t8161: f64, t8251: f64) -> (f64, f64, f64, f64) {
    let t111601 = t30465 * t580;
    let t111602 = t8240 * t1858;
    let t111604 = t576 * t30500;
    let t111636 = t2186 * t671;
    let t111641 = 54.0_f64 * t33185 * t30253 + 54.0_f64 * t29996 * t20176 + 54.0_f64 * t12524 * t30492 + 54.0_f64 * t16524 * t30253 + 54.0_f64 * t75795 * t8251 + 0.135e2_f64 * t8161 * t19534 + 54.0_f64 * t110631 * t5376 + 27.0_f64 * t3941 * t30424 * t671 + 54.0_f64 * t16524 * t30258 + 0.135e2_f64 * t29993 * t5493 + 0.135e2_f64 * t3938 * t30424 + 27.0_f64 * t28893 * t8143 + 0.45e1_f64 * t111594 * t577 + 27.0_f64 * t110684 * t1458 + 27.0_f64 * t12524 * t30495 + 27.0_f64 * t111636 * t5456 + 27.0_f64 * t5371 * t30180;
    (t111601, t111602, t111604, t111641)
}
