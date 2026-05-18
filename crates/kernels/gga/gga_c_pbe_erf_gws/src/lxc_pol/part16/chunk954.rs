//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 954/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk954<F: Float>(t230: F, t2962: F, t1049: F, t678: F, t5938: F, t5940: F, t5944: F, t7567: F, t7569: F, t7572: F, t7576: F, t7578: F, t7581: F, t7584: F, t7593: F, t7595: F, t7597: F, t7599: F) -> F {
    let t8439 = F::new(8.0) / F::new(3.0) * t2962 * t230;
    let t8440 = t1049 * t678;
    let t8442 = F::new(0.43284165449459373508e0) * t5938 + F::new(0.1442805514981979117e0) * t5940 - t5944 + t8439 + F::new(8.0) / F::new(3.0) * t8440 - t7567 - t7569 - t7572 - t7576 + t7578 - t7581 + t7584 + t7593 + t7595 + t7597 + t7599;
    t8442
}
