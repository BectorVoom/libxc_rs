//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1940;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta553<F: Float>(t225: F, t29636: F, t1949: F, t6048: F, t25317: F, t6071: F, t7071: F, t233: F, t1957: F, t1558: F, t231: F, t7759: F, t7076: F, t1580: F, t1956: F, t213: F, t25303: F, t25307: F, t257: F, t27187: F, t27189: F, t27192: F, t27196: F, t27199: F, t27203: F, t27214: F, t27217: F, t29611: F, t6049: F, t6072: F, t7053: F, t7070: F, t7766: F, t7770: F, t7779: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1940::<F>(t225, t29636, t1949, t6048, t25317, t6071, t7071, t233, t1957, t1558, t231, t7759);
        let (t29669, t29672) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1941::<F>(t29668, t7076, t1580, t1956, t213, t25303, t25307, t257, t27187, t27189, t27192, t27196, t27199, t27203, t27214, t27217, t29611, t29637, t29644, t29655, t29659, t6049, t6072, t7053, t7070, t7766, t7770, t7779);
    (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668, t29669, t29672)
}
