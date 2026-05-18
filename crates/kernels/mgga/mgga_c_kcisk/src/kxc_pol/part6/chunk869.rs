//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 869/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk869<F: Float>(t28528: F, t587: F, t1674: F, t22750: F, t2396: F, t28461: F, t28464: F, t28467: F, t28470: F, t28472: F, t28476: F, t28509: F, t6851: F, t8609: F, t8613: F) -> (F, F) {
    let t28530 = F::new(0.62182e-1) * t28528 * t587;
    let t28531 = -t28461 + t28464 - t28467 + t28470 - F::new(0.1025389702100779493e4) * t1674 * t28472 + F::new(0.1038945353962551798e3) * t1674 * t28476 - F::new(0.58482233974552040708e0) * t1674 * t28509 - F::new(0.17544670192365612213e1) * t22750 * t2396 - F::new(0.17544670192365612213e1) * t6851 * t8609 - F::new(0.51947267698127589899e2) * t6851 * t8613 - t28530;
    (t28530, t28531)
}
