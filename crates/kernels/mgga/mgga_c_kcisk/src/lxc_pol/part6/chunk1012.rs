//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1012/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1012<F: Float>(t5730: F, t7764: F, t2083: F, t7757: F, t13009: F, t12969: F, t12941: F, t30233: F, t26: F, t1186: F, t30238: F, t30290: F, t3661: F) -> (F, F, F, F, F, F, F) {
    let t30613 = t5730 * t7764;
    let t30616 = t7757 * t2083;
    let t30617 = t13009 * t30616;
    let t30623 = t12969 * t30616;
    let t30625 = t12941 * t30233;
    let t30626 = t26 * t30625;
    let t30628 = t1186 * t30238;
    let t30629 = t26 * t30628;
    let t30631 = t3661 * t30290;
    (t30613, t30616, t30617, t30623, t30626, t30629, t30631)
}
