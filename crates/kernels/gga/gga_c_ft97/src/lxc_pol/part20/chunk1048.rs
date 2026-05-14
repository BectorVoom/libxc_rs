//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1048/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1048<F: Float>(t24237: F, t28006: F, t1403: F, t27942: F, t681: F, t27968: F, t5996: F, t24188: F, t24231: F, t24234: F, t24425: F, t27894: F, t27943: F, t27963: F, t28012: F, t28015: F, t28042: F, t3051: F, t5995: F, t6002: F, t6064: F, t6745: F, t684: F, t96361: F, t96363: F, t96798: F) -> (F,) {
    let t107806 = t24237 * t28006 / 27.0;
    let t107809 = t1403 * t681 * t27942 / 9.0;
    let t107819 = 2.0 / 9.0 * t5996 * t27968;
    let t107829 = t5996 * t27943 / 3.0 + 2.0 / 9.0 * t5995 * t3051 * t28012 + t107806 - t107809 - 2.0 / 3.0 * t6745 * t24425 - t6745 * t24188 / 3.0 + t27894 * t6064 / 3.0 - 4.0 / 81.0 * t96361 - t96363 / 27.0 + t107819 + 2.0 / 9.0 * t6002 * t24231 * t27963 * t684 + 2.0 / 9.0 * t6002 * t96798 * t28042 + 2.0 / 9.0 * t28015 * t24234;
    (t107829,)
}
