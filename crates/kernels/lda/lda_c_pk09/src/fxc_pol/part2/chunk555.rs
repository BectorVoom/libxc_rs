//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 555/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk555<F: Float>(t3498: F, t831: F, t741: F, t95: F, t568: F, t952: F, t205: F, t198: F, t733: F, t142: F, t3163: F, t957: F) -> (F, F, F, F, F) {
    let t3500 = F::new(24.847010690177285) * t831 * t3498;
    let t3501 = t741 * t95;
    let t3506 = t568 * t952;
    let t3507 = t205 * t3506;
    let t3509 = t198 * t733;
    let t3510 = t3509 * t142;
    let t3512 = F::new(4.855032390388656) * t3510 * t3163;
    let t3514 = F::new(3.2366882602591036) * t957 * t3498;
    (t3500, t3501, t3507, t3512, t3514)
}
