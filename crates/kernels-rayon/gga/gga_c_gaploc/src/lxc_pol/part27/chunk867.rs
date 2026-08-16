//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 867/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk867(t1457: f64, t8612: f64, t1865: f64, t2949: f64, t1445: f64, t2959: f64, t4614: f64, t4673: f64, t1998: f64, t2103: f64, t2684: f64, t3025: f64, t3032: f64, t3035: f64, t5724: f64, t5748: f64, t6060: f64, t6159: f64, t7567: f64, t7570: f64, t7580: f64, t7582: f64, t7627: f64, t7769: f64, t807: f64, t813: f64, t833: f64, t8580: f64, t8588: f64, t8592: f64, t8595: f64, t8600: f64, t8601: f64, t8606: f64) -> f64 {
    let t8613 = t1457 * t8612;
    let t8616 = t2949 * t1865;
    let t8617 = t1445 * t8616;
    let t8620 = t4614 * t2959;
    let t8623 = t4673 * t2959;
    let t8626 = 0.29792074959875355558e-1_f64 * t7567 + 0.11916829983950142223e0_f64 * t7570 - 0.19171462976960374838e0_f64 * t7580 + 0.85206502119823888168e-1_f64 * t7582 - 0.59644551483876721719e0_f64 * t7627 + 0.11360866949309851756e0_f64 * t2684 * t8580 - 0.35750489951850426669e0_f64 * t3035 * t5724 - 0.46011511144704899612e1_f64 * t6159 * t3032 - 0.46011511144704899612e1_f64 * t1998 * t8588 - 0.23005755572352449806e1_f64 * t1998 * t8592 + 0.46011511144704899612e1_f64 * t807 * t8595 - 0.25025342966295298669e1_f64 * t3025 * t7769 + 0.43710935587469654631e2_f64 * t833 * t8601 + 0.27606906686822939767e2_f64 * t5748 * t8606 + 0.42900587942220512003e1_f64 * t2103 * t1457 * t8600 - 0.21450293971110256001e1_f64 * t6060 * t8613 - 0.92023022289409799224e1_f64 * t813 * t8617 + 0.30674340763136599741e2_f64 * t833 * t8620 + 0.95334639871601137784e0_f64 * t2103 * t8623;
    t8626
}
