//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 841/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk841<F: Float>(t30829: F, t31769: F, t544: F, t913: F, t1424: F, t2875: F, t9060: F, t40202: F, t3177: F, t8272: F, t9267: F, t40208: F) -> (F, F, F, F, F) {
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41885 = F::new(0.3575048995185042667e0) * t41884;
    let t41889 = F::new(0.39722766613167140743e-1) * t544 * t9060 * t2875 * t1424;
    let t41893 = F::new(0.46011511144704899612e1) * t40202;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = F::new(0.19171462976960374838e1) * t41903;
    let t41905 = F::new(0.10352590007558602413e2) * t40208;
    (t41885, t41889, t41893, t41904, t41905)
}
