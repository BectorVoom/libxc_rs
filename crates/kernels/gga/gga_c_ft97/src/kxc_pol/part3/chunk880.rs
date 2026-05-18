//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 880/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk880<F: Float>(t17612: F, t17673: F, t184: F, t21: F, t15625: F, t17524: F, t17532: F, t17535: F, t17539: F, t17542: F, t17545: F, t185: F, t3597: F, t3601: F, t363: F, t3674: F, t3678: F, t4431: F, t4845: F, t5: F, t620: F, t623: F, t920: F) -> F {
    let t17674 = t17612 + t17673;
    let t17675 = t17674 * t184;
    let t17676 = t17675 * t21;
    let t17679 = t3601 * t3674 / F::new(2.0) + t3601 * t3678 + t5 * t3597 * t920 / F::new(2.0) + t5 * t185 * t15625 / F::new(4.0) + t5 * t620 * t4431 / F::new(4.0) + t5 * t17524 * t21 / F::new(4.0) + t5 * t4845 * t363 / F::new(4.0) + t623 * t17532 / F::new(2.0) + t623 * t17535 / F::new(4.0) + t623 * t17539 / F::new(2.0) + t623 * t17542 + t623 * t17545 / F::new(4.0) + t623 * t17676 / F::new(4.0);
    t17679
}
