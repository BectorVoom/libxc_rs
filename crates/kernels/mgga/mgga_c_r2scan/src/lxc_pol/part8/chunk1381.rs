//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1381/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1381<F: Float>(t21837: F, t21843: F, t21846: F, t21858: F, t22382: F, t22386: F, t22390: F, t229: F, t26750: F, t26752: F, t26755: F, t26756: F, t26758: F, t32195: F, t33642: F, t41: F) -> (F,) {
    let t33646 = -0.11711067804325879029e1 * t26750 + 0.17544670867903938621e1 * t26752 + t26755 - t33642 - t41 * t32195 * t229 + t21837 + t21843 + t22382 + t21846 + t22386 + t26756 - t26758 + 0.10526802520742363173e2 * t22390 - t21858;
    (t33646,)
}
