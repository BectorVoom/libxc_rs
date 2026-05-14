//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 897/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk897<F: Float>(t5730: F, t7764: F, t2083: F, t7757: F, t13009: F, t12969: F, t12941: F, t30233: F, t26: F, t1186: F, t30238: F, t30290: F, t3661: F, t30298: F, t12999: F, t13000: F, t19543: F, t30592: F, t30595: F, t30599: F, t30603: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30613 = t5730 * t7764;
    let t30616 = t7757 * t2083;
    let t30617 = t13009 * t30616;
    let t30623 = t12969 * t30616;
    let t30625 = t12941 * t30233;
    let t30626 = t26 * t30625;
    let t30628 = t1186 * t30238;
    let t30629 = t26 * t30628;
    let t30631 = t3661 * t30290;
    let t30632 = t26 * t30631;
    let t30634 = t1186 * t30298;
    let t30635 = t26 * t30634;
    let t30637 = 0.46074375e0 * t30613 - t12999 - t13000 - 0.27385555555555555556e0 * t19543 + 0.142419375e1 * t30617 + 0.11958666666666666667e1 * t30595 - 0.17938e1 * t30599 - 0.33218518518518518518e0 * t30592 - 0.29896666666666666667e0 * t30603 - 0.76790625e-1 * t30623 - 0.36514074074074074075e-1 * t30626 - 0.82156666666666666667e-1 * t30629 + 0.16431333333333333333e0 * t30632 - 0.49293999999999999999e0 * t30635;
    (t30613, t30616, t30617, t30623, t30626, t30629, t30632, t30635, t30637)
}
