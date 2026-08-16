//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta561<F: Float>(t27349: F, t689: F, t25260: F, t4368: F, t820: F, t844: F, t4462: F, t92951: F, t92963: F, t92966: F, t92969: F, t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F, t14857: F, t25234: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98892, t98937, t98949, t98960, t98961, t98962, t98964) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880::<F>(t27349, t689, t25260, t4368, t820, t844, t4462, t92951, t92963, t92966, t92969, t27253, t9775);
        let (t98968, t98972, t98976, t98979) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1881::<F>(t14833, t240, t2661, t7043, t14857, t25234, t25240, t2710, t4371, t10744, t4353, t7028);
    (t98892, t98937, t98949, t98960, t98961, t98962, t98964, t98968, t98972, t98976, t98979)
}
