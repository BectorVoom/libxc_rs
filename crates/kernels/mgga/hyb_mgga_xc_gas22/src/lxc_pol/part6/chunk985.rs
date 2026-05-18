//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 985/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk985<F: Float>(t9135: F, t950: F, t957: F, t2490: F, t3485: F, t2496: F, t3490: F, t952: F, t3496: F, t6969: F, t6972: F, t9113: F, t9116: F, t9119: F, t9123: F, t9127: F) -> (F, F, F, F, F, F) {
    let t9136 = t950 * t9135;
    let t9138 = t957 * t9135;
    let t9140 = t3485 * t2490;
    let t9142 = t2496 * t3490;
    let t9143 = t9142 * t952;
    let t9145 = t3496 * t2490;
    let t9147 = -t9113 - t9116 + F::new(0.24647e0) * t9119 + F::new(0.49294e0) * t9123 + F::new(0.24647e0) * t9127 + F::new(0.79724444444444444446e0) * t6969 - F::new(0.29896666666666666667e0) * t6972 + F::new(0.1898925e1) * t9136 + F::new(0.3071625e0) * t9138 - F::new(0.9494625e0) * t9140 + F::new(0.3071625e0) * t9143 + F::new(0.15358125e0) * t9145;
    (t9136, t9138, t9140, t9143, t9145, t9147)
}
