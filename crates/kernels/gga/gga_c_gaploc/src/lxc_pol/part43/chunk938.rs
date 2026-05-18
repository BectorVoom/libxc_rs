//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 938/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk938<F: Float>(t1457: F, t43240: F, t6060: F, t13158: F, t15766: F, t41425: F, t41430: F, t41435: F, t41445: F, t24549: F, t7584: F, t9438: F) -> (F, F, F, F, F, F, F) {
    let t44097 = F::new(0.21450293971110256001e1) * t6060 * t1457 * t43240;
    let t44099 = F::new(0.21450293971110256001e1) * t15766 * t13158;
    let t44106 = F::new(0.1022478025437886658e1) * t41425;
    let t44110 = F::new(0.19171462976960374838e1) * t41430;
    let t44111 = F::new(0.42603251059911944084e0) * t41435;
    let t44112 = F::new(0.29792074959875355558e-1) * t41445;
    let t44117 = t7584 * t9438 * t24549;
    (t44097, t44099, t44106, t44110, t44111, t44112, t44117)
}
