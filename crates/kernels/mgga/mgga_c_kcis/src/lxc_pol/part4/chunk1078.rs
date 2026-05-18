//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1078/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1078<F: Float>(t1072: F, t4833: F, t331: F, t4837: F, t1717: F, t2635: F, t4840: F, t829: F, t3096: F, t4836: F, t1035: F, t167: F) -> (F, F, F, F, F, F) {
    let t13665 = F::new(0.93706135855523581992e-2) * t1072 * t4833;
    let t13667 = F::new(0.93706135855523581992e-2) * t331 * t4837;
    let t13668 = t1717 * t2635;
    let t13671 = t4840 * t829;
    let t13674 = t4836 * t3096;
    let t13677 = t1035 * t167;
    (t13665, t13667, t13668, t13671, t13674, t13677)
}
