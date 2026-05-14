//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 964/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk964<F: Float>(t26706: F, t32152: F, t26722: F, t26715: F, t32233: F, t138738: F, t3392: F, t26696: F, t1008: F, t7189: F, t137007: F, t554: F, t1013: F, t538: F, t26701: F, t32174: F, t34877: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t147231 = t32152 * t26706;
    let t147234 = t32152 * t26722;
    let t147238 = t32233 * t26715;
    let t147243 = t3392 * t138738;
    let t147248 = t32152 * t26696;
    let t147251 = t7189 * t1008;
    let t147253 = t137007 * t147251 * t554;
    let t147256 = t7189 * t1013;
    let t147258 = t137007 * t147256 * t538;
    let t147262 = t32152 * t26701;
    let t147266 = t32174 * t34877;
    (t147231, t147234, t147238, t147243, t147248, t147253, t147256, t147258, t147262, t147266)
}
