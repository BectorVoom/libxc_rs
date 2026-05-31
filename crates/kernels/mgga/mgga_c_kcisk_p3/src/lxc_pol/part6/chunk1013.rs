//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1013/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1013<F: Float>(t26: F, t30631: F, t1186: F, t30298: F, t12999: F, t13000: F, t19543: F, t30592: F, t30595: F, t30599: F, t30603: F, t30613: F, t30617: F, t30623: F, t30626: F, t30629: F) -> (F, F, F) {
    let t30632 = t26 * t30631;
    let t30634 = t1186 * t30298;
    let t30635 = t26 * t30634;
    let t30637 = F::cast_from(0.46074375e0_f64) * t30613 - t12999 - t13000 - F::cast_from(0.27385555555555555556e0_f64) * t19543 + F::cast_from(0.142419375e1_f64) * t30617 + F::cast_from(0.11958666666666666667e1_f64) * t30595 - F::cast_from(0.17938e1_f64) * t30599 - F::cast_from(0.33218518518518518518e0_f64) * t30592 - F::cast_from(0.29896666666666666667e0_f64) * t30603 - F::cast_from(0.76790625e-1_f64) * t30623 - F::cast_from(0.36514074074074074075e-1_f64) * t30626 - F::cast_from(0.82156666666666666667e-1_f64) * t30629 + F::cast_from(0.16431333333333333333e0_f64) * t30632 - F::cast_from(0.49293999999999999999e0_f64) * t30635;
    (t30632, t30635, t30637)
}
