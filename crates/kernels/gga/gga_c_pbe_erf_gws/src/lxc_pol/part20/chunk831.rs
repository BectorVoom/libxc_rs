//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 831/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk831<F: Float>(t211: F, t7844: F, t1798: F, t2741: F, t219: F, t5400: F, t2584: F, t5125: F, t1820: F, t2666: F, t5137: F, t639: F) -> (F, F, F, F, F) {
    let t7845 = t211 * t7844;
    let t7852 = F::new(16.0) / F::new(45.0) * t2741 * t1798;
    let t7853 = t5400 * t219;
    let t7868 = t5125 * t2584;
    let t7870 = F::new(32.0) / F::new(135.0) * t1820 * t7868;
    let t7871 = t5137 * t2666;
    let t7873 = F::new(16.0) / F::new(135.0) * t639 * t7871;
    (t7845, t7852, t7853, t7870, t7873)
}
