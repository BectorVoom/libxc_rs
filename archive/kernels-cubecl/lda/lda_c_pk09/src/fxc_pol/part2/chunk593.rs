//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 593/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk593<F: Float>(t151: F, t3557: F, t155: F, t3498: F, t719: F, t190: F, t733: F, t142: F, t3163: F, t187: F, t3995: F, t204: F) -> (F, F, F, F, F) {
    let t4625 = F::cast_from(2.089485677430591_f64) * t151 * t3557;
    let t4627 = F::cast_from(21.65463752731128_f64) * t155 * t3557;
    let t4633 = F::cast_from(2.9520998905110765_f64) * t719 * t3498;
    let t4640 = t190 * t733;
    let t4641 = t4640 * t142;
    let t4643 = F::cast_from(4.4281498357666145_f64) * t4641 * t3163;
    let t4644 = t187 * t3995;
    let t4645 = t4644 * t204;
    (t4625, t4627, t4633, t4643, t4645)
}
