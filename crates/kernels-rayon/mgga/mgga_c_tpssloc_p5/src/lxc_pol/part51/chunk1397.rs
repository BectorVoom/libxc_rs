//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1397/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1397(t1509: f64, t8543: f64, t1888: f64, t232: f64, t6646: f64, t92552: f64, t26676: f64, t33384: f64, t6547: f64, t118578: f64, t112778: f64, t112784: f64, t112804: f64, t118533: f64, t118535: f64, t118539: f64, t118546: f64, t118549: f64, t118552: f64, t118556: f64, t118559: f64, t118562: f64, t118566: f64, t118569: f64, t118573: f64, t118576: f64, t118580: f64) -> (f64, f64, f64, f64, f64) {
    let t121553 = t8543 * t1509;
    let t121560 = t1888 * t6646 * t92552 * t232;
    let t121563 = t1888 * t6646 * t26676;
    let t121574 = t6547 * t33384;
    let t121591 = 0.11304371706359309439e-1_f64 * t118578;
    let t121593 = -t118533 / 768.0_f64 - t118535 / 768.0_f64 - t118539 / 768.0_f64 + 5.0_f64 / 192.0_f64 * t118546 - 0.16149102437656156341e-2_f64 * t118549 + 0.67826230238155856632e-1_f64 * t118552 + 0.26915170729426927235e-3_f64 * t112778 + 0.32298204875312312682e-2_f64 * t118556 + 0.96894614625936938046e-2_f64 * t118559 + 0.67826230238155856634e-1_f64 * t112784 + t118562 / 384.0_f64 + t112804 + 0.96894614625936938046e-2_f64 * t118566 - 0.16149102437656156341e-2_f64 * t118569 + 0.16149102437656156341e-2_f64 * t118573 + t118576 / 768.0_f64 + t121591 + 0.67826230238155856632e-1_f64 * t118580;
    (t121553, t121560, t121563, t121574, t121593)
}
