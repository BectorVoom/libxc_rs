//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1021/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1021<F: Float>(t41542: F, t41528: F, t41532: F, t41534: F, t41544: F, t44154: F, t44155: F, t44156: F, t44157: F, t44159: F, t44162: F, t44164: F, t44167: F, t44170: F, t44174: F, t44178: F, t44179: F, t44180: F, t44181: F, t44185: F) -> F {
    let t44186 = F::new(0.25561950635947166451e0) * t41542;
    let t44188 = -t44154 + t44155 - t44156 - t44157 + F::new(0.95334639871601137787e0) * t44159 - t44162 - t44164 - F::new(0.13803453343411469884e2) * t44167 - t44170 - t44174 - t44178 - t44179 - t44180 + t44181 - F::new(0.38342925953920749676e0) * t41528 + F::new(0.85206502119823888169e-1) * t41532 - F::new(0.38342925953920749676e0) * t41534 + t44185 + t44186 - F::new(0.76685851907841499352e0) * t41544;
    t44188
}
