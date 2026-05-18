//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 609/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk609<F: Float>(t2497: F, t4917: F, t737: F, t4635: F, t738: F, t192: F, t2506: F, t4934: F, t5053: F, t743: F, t2481: F, t3908: F, t3925: F, t462: F, t5099: F, t5102: F, t92: F) -> (F, F, F, F, F, F, F) {
    let t5105 = t2497 * t4917;
    let t5106 = t737 * t5105;
    let t5109 = t738 * t4635;
    let t5110 = t737 * t5109;
    let t5114 = t192 * t2506 * t4934;
    let t5118 = t192 * t743 * t5053;
    let t5120 = t2481 + F::new(2.0) / F::new(9.0) * t3908 + F::new(2.0) / F::new(3.0) * t3925 - F::new(2.0) / F::new(9.0) * t462 * t5099 + F::new(2.0) / F::new(3.0) * t462 * t5102 + F::new(2.0) / F::new(3.0) * t462 * t5106 - t462 * t5110 / F::new(3.0) + F::new(2.0) * t92 * t5114 - t92 * t5118;
    (t5105, t5106, t5109, t5110, t5114, t5118, t5120)
}
