//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1154/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1154<F: Float>(t13879: F, t1897: F, t702: F, t13941: F, t2508: F, t779: F, t42980: F, t42981: F, t42982: F, t42983: F, t42984: F, t42986: F, t42989: F, t42992: F, t42998: F) -> F {
    let t47616 = F::new(0.76905262301422242837e-2) * t1897 * t13879 * t702;
    let t47619 = F::new(0.76905262301422242837e-2) * t2508 * t779 * t13941;
    let t47620 = -t42980 - t42981 + t42982 - t42983 + t42984 + t42986 + t42989 + t42992 - t47616 + t47619 - t42998;
    t47620
}
