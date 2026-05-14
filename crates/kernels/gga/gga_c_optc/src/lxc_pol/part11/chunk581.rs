//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 581/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk581<F: Float>(t2476: F, t4854: F, t1355: F, t1367: F, t2493: F, t2518: F, t252: F, t2530: F, t2537: F, t3716: F, t3754: F, t4781: F, t4785: F, t4817: F, t4821: F, t4863: F, t4869: F, t4885: F, t4888: F, t4897: F, t4900: F, t4904: F, t4920: F, t810: F, t829: F) -> (F, F) {
    let t4923 = t4854 * t2476;
    let t4926 = -0.3109e-1 * t4863 * t252 + 2.0 * t3716 * t1355 - 2.0 * t2493 * t4869 + 1.0 * t810 * t4885 + 0.32164683177870697974e2 * t2518 * t4888 + t4897 - t4785 + t4900 - t4817 - t4821 - 0.19751789702565206229e-1 * t4781 + 0.11696446794910408142e1 * t3754 * t1367 - 0.11696446794910408142e1 * t2530 * t4904 + 0.58482233974552040708e0 * t829 * t4920 + 0.17315755899375863299e2 * t2537 * t4923;
    (t4923, t4926)
}
