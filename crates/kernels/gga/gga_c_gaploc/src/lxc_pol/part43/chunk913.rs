//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 913/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk913<F: Float>(t3040: F, t41236: F, t1022: F, t9755: F, t2639: F, t787: F, t28002: F, t9858: F, t13141: F, t2464: F, t2684: F, t32809: F, t32810: F, t43494: F) -> (F, F, F, F, F, F) {
    let t43571 = F::new(0.35750489951850426669e0) * t41236 * t3040;
    let t43572 = t9755 * t1022;
    let t43575 = F::new(0.53625734927775640005e1) * t787 * t43572 * t2639;
    let t43579 = F::new(0.17875244975925213335e2) * t787 * t28002 * t1022 * t9858;
    let t43581 = t2684 * t2464 * t13141;
    let t43582 = F::new(0.63904876589867916128e-1) * t43581;
    let t43592 = F::new(0.85801175884441024004e1) * t32809 * t32810 * t43494;
    (t43571, t43572, t43575, t43579, t43582, t43592)
}
