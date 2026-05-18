//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1293/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1293<F: Float>(t11029: F, t2087: F, t4614: F, t10951: F, t5782: F, t2101: F, t3431: F, t1890: F, t3487: F, t7805: F, t7810: F, t107: F, t10809: F, t787: F) -> (F, F, F, F, F) {
    let t33282 = F::new(0.18404604457881959845e2) * t2087 * t4614 * t11029;
    let t33284 = F::new(0.18404604457881959845e2) * t5782 * t10951;
    let t33285 = t2101 * t3431;
    let t33289 = t1890 * t3487;
    let t33291 = t7810 * t33289 * t7805;
    let t33292 = F::new(0.19171462976960374838e1) * t33291;
    let t33294 = t787 * t10809 * t107;
    (t33282, t33284, t33285, t33292, t33294)
}
