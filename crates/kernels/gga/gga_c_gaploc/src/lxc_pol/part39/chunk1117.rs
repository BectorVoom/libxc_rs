//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1117/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1117<F: Float>(t32970: F, t47266: F, t13870: F, t835: F, t723: F, t1457: F, t2103: F, t13900: F, t5771: F, t43605: F, t43606: F, t43607: F, t43609: F, t43611: F, t43617: F, t47255: F, t47261: F, t47263: F) -> (F, F, F) {
    let t47267 = t47266 * t32970;
    let t47270 = t835 * t13870;
    let t47271 = t47270 * t723;
    let t47274 = F::new(0.71500979903700853338e0) * t2103 * t1457 * t47271;
    let t47275 = t5771 * t13900;
    let t47277 = -t43605 + t43606 - t43607 - F::new(0.46011511144704899612e1) * t47255 - t47261 - F::new(0.92023022289409799224e1) * t47263 + F::new(0.19171462976960374838e0) * t43609 - F::new(0.25025342966295298669e1) * t47267 + F::new(0.19171462976960374838e0) * t43611 + t47274 + F::new(0.71500979903700853338e0) * t47275 + t43617;
    (t47270, t47271, t47277)
}
