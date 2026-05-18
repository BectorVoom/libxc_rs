//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 902/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk902<F: Float>(t44888: F, t701: F, t326: F, t45320: F, t825: F, t10930: F, t10931: F, t723: F, t1457: F, t2645: F, t36516: F, t43464: F) -> (F, F, F, F, F, F) {
    let t45337 = t44888 * t701;
    let t45343 = F::new(0.18404604457881959845e2) * t825 * t326 * t45320;
    let t45349 = F::new(0.55213813373645879534e2) * t10930 * t10931 * t45320;
    let t45350 = t44888 * t723;
    let t45356 = F::new(0.42900587942220512003e1) * t36516 * t1457 * t2645;
    let t45357 = F::new(0.11916829983950142223e0) * t43464;
    (t45337, t45343, t45349, t45350, t45356, t45357)
}
