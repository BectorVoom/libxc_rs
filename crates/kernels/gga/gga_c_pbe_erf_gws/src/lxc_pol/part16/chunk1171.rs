//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1171/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1171<F: Float>(t14188: F, t26958: F, t353: F, t4228: F, t814: F, t859: F, t14888: F, t19906: F, t1206: F, t3200: F, t335: F, t338: F, t4111: F, t4385: F, t51807: F, t51819: F, t51827: F, t51829: F, t52241: F, t52600: F, t53910: F, t53925: F, t53930: F, t53936: F, t8629: F, t8793: F, t8939: F, t9241: F, t9283: F) -> (F,) {
    let t55695 = 7.0 / 72.0 * t26958 * t14188;
    let t55698 = t859 * t353 * t4228 * t814;
    let t55702 = 7.0 / 72.0 * t19906 * t14888;
    let t55703 = t9241 * t9283 * t1206 * t8939 / 4.0 - t335 * t338 * t3200 * t4111 / 48.0 + 7.0 / 2304.0 * t51807 - t53910 / 48.0 - 119.0 / 3456.0 * t51819 + 7.0 / 2304.0 * t51827 - 7.0 / 288.0 * t51829 - t53925 / 6.0 + t8629 * t52600 / 96.0 + t53930 / 96.0 - t53936 / 384.0 - t8793 * t52241 / 16.0 - t55695 + t4385 * t55698 / 96.0 - t55702;
    (t55703,)
}
