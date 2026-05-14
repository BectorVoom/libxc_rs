//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1113/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1113<F: Float>(t1403: F, t27946: F, t681: F, t27893: F, t92: F, t27915: F, t5996: F, t1401: F, t1900: F, t7149: F, t6003: F, t9895: F, t42123: F, t263: F, t27742: F, t1425: F, t9568: F) -> (F, F, F, F, F, F, F, F) {
    let t107835 = t1403 * t681 * t27946 / 9.0;
    let t107836 = t27893 * t92;
    let t107871 = t5996 * t27915 / 9.0;
    let t107885 = t1401 * t7149 * t1900;
    let t107886 = t9895 * t6003;
    let t107893 = t42123 * t6003;
    let t107910 = t27742 * t263;
    let t107919 = t9568 * t1425;
    (t107835, t107836, t107871, t107885, t107886, t107893, t107910, t107919)
}
