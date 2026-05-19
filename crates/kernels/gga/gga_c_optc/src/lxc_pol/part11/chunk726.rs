//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk726<F: Float>(t3086: F, t8414: F, t1113: F, t2849: F, t195: F, t429: F, t116: F, t428: F, t3016: F, t385: F, t375: F, t373: F) -> (F, F, F, F, F, F, F, F) {
    let t8532 = t3086 * t8414;
    let t8537 = t1113 * t2849;
    let t8545 = t195 * t429;
    let t8546 = t116 * t8545;
    let t8548 = F::new(5.0) / F::new(1296.0) * t428 * t8546;
    let t8581 = F::new(1.0) / t3016 / t385;
    let t8582 = t375 * t8581;
    let t8611 = F::new(1.0)/pow_3_2::<F>(t373);
    (t8532, t8537, t8545, t8546, t8548, t8581, t8582, t8611)
}
