//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 928/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk928<F: Float>(t13665: F, t13522: F, t13533: F, t13536: F, t13540: F, t13543: F, t13549: F, t13555: F, t13650: F, t13653: F, t13656: F, t13659: F, t13662: F, t13670: F) -> F {
    let t13746 = F::new(0.73586666666666666667e0) * t13665;
    let t13748 = F::new(0.93932222222222222223e0) * t13522;
    let t13749 = -F::new(0.60385000000000000001e0) * t13533 + F::new(0.30192500000000000001e0) * t13536 - F::new(0.33547222222222222222e0) * t13540 + F::new(0.12077e1) * t13543 - F::new(0.181155e1) * t13549 - F::new(0.301925e0) * t13555 + F::new(0.33114e0) * t13650 - F::new(0.3883875e1) * t13653 + F::new(0.247573125e0) * t13656 - F::new(0.16557e0) * t13659 + F::new(0.99342e0) * t13662 - t13746 + F::new(0.19419375e1) * t13670 - t13748;
    t13749
}
