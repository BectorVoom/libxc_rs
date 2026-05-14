//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1050/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1050<F: Float>(t27915: F, t5996: F, t1401: F, t1900: F, t7149: F, t6003: F, t9895: F, t10002: F, t27924: F, t42123: F, t13702: F, t13757: F, t1403: F, t193: F, t24197: F, t24201: F, t24204: F, t24228: F, t24247: F, t28015: F, t28043: F, t4003: F, t6062: F, t96397: F, t96400: F, t96770: F, t96782: F) -> (F, F, F) {
    let t107871 = t5996 * t27915 / 9.0;
    let t107885 = t1401 * t7149 * t1900;
    let t107886 = t9895 * t6003;
    let t107891 = t10002 * t27924;
    let t107893 = t42123 * t6003;
    let t107901 = -t107871 + 2.0 / 9.0 * t24204 * t28043 + 4.0 / 27.0 * t96397 + 4.0 / 27.0 * t96400 - t28015 * t24247 / 9.0 - t28015 * t24197 / 18.0 - t28015 * t24201 / 27.0 - t96770 + t28015 * t24228 / 9.0 - 4.0 / 9.0 * t107885 * t107886 * t13702 + 4.0 / 27.0 * t96782 + 8.0 * t107891 - 2.0 / 3.0 * t107885 * t107893 * t13757 + t1403 * t193 * t6062 * t4003 / 3.0;
    (t107885, t107891, t107901)
}
