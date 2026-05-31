//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1403/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1403<F: Float>(t21832: F, t21836: F, t21840: F, t21845: F, t21850: F, t21856: F, t21894: F, t21932: F, t25937: F, t25939: F, t25941: F, t25944: F, t25946: F, t25948: F, t25951: F, t25954: F, t25957: F) -> F {
    let t30387 = t21832 + F::cast_from(0.97661052298701573622e-3_f64) * t25937 - F::cast_from(0.43374325201206959368e-1_f64) * t25939 + F::cast_from(0.96319466275353142155e0_f64) * t25941 + F::cast_from(0.43374325201206959368e-1_f64) * t25944 + F::cast_from(0.32530743900905219526e-1_f64) * t25946 - F::cast_from(0.65061487801810439052e-1_f64) * t25948 - t21836 - t21840 - t21845 + t21850 + F::cast_from(2.0_f64) * t25951 - t21856 + F::cast_from(4.0_f64) * t25954 + F::cast_from(2.0_f64) * t25957 + t21894 + t21932;
    t30387
}
