//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1164/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1164<F: Float>(t23759: F, t31540: F, t10241: F, t161: F, t1353: F, t23767: F, t1061: F, t424: F, t481: F, t6603: F, t7974: F, t10167: F, t1358: F) -> (F, F, F, F, F, F) {
    let t31542 = F::new(0.12646669615856066488e-1) * t23759 * t31540;
    let t31543 = t10241 * t161;
    let t31546 = F::new(0.63233348079280332442e-2) * t23767 * t31543 * t1353;
    let t31548 = t481 * t1061 * t424;
    let t31551 = F::new(0.56910013271352299198e-1) * t31548 * t6603 * t7974;
    let t31552 = t1358 * t10167;
    (t31542, t31543, t31546, t31548, t31551, t31552)
}
