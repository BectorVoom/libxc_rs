//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 620/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk620<F: Float>(t1315: F, t4979: F, t1310: F, t5031: F, t1287: F, t1307: F, t5081: F, t347: F, t4998: F, t1468: F, t300: F, t1284: F) -> (F, F, F, F, F, F) {
    let t5095 = F::new(3.7610742193750633) * t1315 * t4979;
    let t5103 = t1310 * t5031;
    let t5104 = t5103 * t1287;
    let t5106 = t1307 * t5081;
    let t5108 = t347 * t5031;
    let t5115 = F::new(2.507382812916709) * t1315 * t4998;
    let t5116 = t300 * t1468;
    let t5117 = t5116 * t1284;
    (t5095, t5104, t5106, t5108, t5115, t5117)
}
