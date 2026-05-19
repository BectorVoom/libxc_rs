//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 865/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk865<F: Float>(t1457: F, t8612: F, t1865: F, t2949: F, t1445: F, t2959: F, t4614: F, t4673: F, t1998: F, t2103: F, t2684: F, t3025: F, t3032: F, t3035: F, t5724: F, t5748: F, t6060: F, t6159: F, t7567: F, t7570: F, t7580: F, t7582: F, t7627: F, t7769: F, t807: F, t813: F, t833: F, t8580: F, t8588: F, t8592: F, t8595: F, t8600: F, t8601: F, t8606: F) -> F {
    let t8613 = t1457 * t8612;
    let t8616 = t2949 * t1865;
    let t8617 = t1445 * t8616;
    let t8620 = t4614 * t2959;
    let t8623 = t4673 * t2959;
    let t8626 = F::cast_from(0.29792074959875355558e-1_f64) * t7567 + F::cast_from(0.11916829983950142223e0_f64) * t7570 - F::cast_from(0.19171462976960374838e0_f64) * t7580 + F::cast_from(0.85206502119823888168e-1_f64) * t7582 - F::cast_from(0.59644551483876721719e0_f64) * t7627 + F::cast_from(0.11360866949309851756e0_f64) * t2684 * t8580 - F::cast_from(0.35750489951850426669e0_f64) * t3035 * t5724 - F::cast_from(0.46011511144704899612e1_f64) * t6159 * t3032 - F::cast_from(0.46011511144704899612e1_f64) * t1998 * t8588 - F::cast_from(0.23005755572352449806e1_f64) * t1998 * t8592 + F::cast_from(0.46011511144704899612e1_f64) * t807 * t8595 - F::cast_from(0.25025342966295298669e1_f64) * t3025 * t7769 + F::cast_from(0.43710935587469654631e2_f64) * t833 * t8601 + F::cast_from(0.27606906686822939767e2_f64) * t5748 * t8606 + F::cast_from(0.42900587942220512003e1_f64) * t2103 * t1457 * t8600 - F::cast_from(0.21450293971110256001e1_f64) * t6060 * t8613 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t8617 + F::cast_from(0.30674340763136599741e2_f64) * t833 * t8620 + F::cast_from(0.95334639871601137784e0_f64) * t2103 * t8623;
    t8626
}
