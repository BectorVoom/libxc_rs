//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1049/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1049<F: Float>(t26261: F, t26264: F, t2972: F, t393: F, t2975: F, t26224: F, t406: F, t26214: F, t1135: F, t508: F, t438: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26599 = F::new(0.5356037037037037037e1) * t26261;
    let t26600 = F::new(0.16979925925925925926e1) * t26264;
    let t26663 = t2972 * t2972;
    let t26665 = t393 / t26663;
    let t26666 = t2975 * t2975;
    let t26667 = F::new(1.0) / t26666;
    let t26738 = t406 * t26224;
    let t26745 = t406 * t26214;
    let t26780 = F::new(0.96141975308641975307e-1) * t26261;
    let t26808 = F::new(0.17757530864197530864e0) * t26261;
    let t26836 = F::new(0.18467901234567901234e0) * t26261;
    let t26869 = t508 * t1135;
    let t26881 = t935 * t438;
    (t26599, t26600, t26665, t26667, t26738, t26745, t26780, t26808, t26836, t26869, t26881)
}
