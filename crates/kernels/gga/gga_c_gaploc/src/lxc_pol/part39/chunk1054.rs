//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1054/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1054<F: Float>(t11001: F, t2714: F, t2718: F, t33725: F, t955: F, t10054: F, t3040: F, t3267: F, t8556: F, t13126: F, t2194: F, t1445: F, t2087: F, t43240: F) -> (F, F, F, F, F, F, F) {
    let t44020 = t2714 * t11001;
    let t44022 = t2718 * t11001;
    let t44024 = t955 * t33725;
    let t44027 = F::new(0.35750489951850426669e0) * t10054 * t3040;
    let t44029 = F::new(0.23833659967900284446e0) * t3267 * t8556;
    let t44030 = t2194 * t13126;
    let t44038 = F::new(0.62115540045351614476e2) * t2087 * t1445 * t43240;
    (t44020, t44022, t44024, t44027, t44029, t44030, t44038)
}
