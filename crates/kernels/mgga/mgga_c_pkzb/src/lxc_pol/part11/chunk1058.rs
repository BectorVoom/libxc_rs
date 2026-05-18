//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1058/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1058<F: Float>(t1531: F, t466: F, t5152: F, t4868: F, t4871: F, t4885: F, t501: F, t1497: F, t1503: F, t1507: F, t555: F, t1511: F, t5146: F) -> (F, F, F, F, F, F) {
    let t16578 = F::new(0.38025319932552508021e2) * t1531 * t466 * t5152;
    let t16582 = t4871 * t4868;
    let t16584 = t501 * t4885;
    let t16588 = t1497 * t1497;
    let t16592 = F::new(0.51947577317044391277e2) * t555 * t1503 * t16588 * t1507;
    let t16593 = t1511 * t5146;
    (t16578, t16582, t16584, t16588, t16592, t16593)
}
