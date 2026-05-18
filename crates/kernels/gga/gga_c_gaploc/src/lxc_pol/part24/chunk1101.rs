//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1101/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1101<F: Float>(t28563: F, t2586: F, t2617: F, t7803: F, t7344: F, t948: F, t20671: F, t22543: F, t22980: F, t21461: F, t2365: F, t7390: F) -> (F, F, F, F, F) {
    let t28564 = F::new(0.76685851907841499352e0) * t28563;
    let t28566 = t7803 * t2586 * t2617;
    let t28567 = F::new(0.76685851907841499352e0) * t28566;
    let t28569 = t7803 * t948 * t7344;
    let t28570 = F::new(0.38342925953920749676e0) * t28569;
    let t28585 = F::new(0.17041300423964777634e0) * t22543 * t20671 * t22980;
    let t28593 = F::new(0.29792074959875355558e-1) * t7390 * t2365 * t21461;
    (t28564, t28567, t28570, t28585, t28593)
}
