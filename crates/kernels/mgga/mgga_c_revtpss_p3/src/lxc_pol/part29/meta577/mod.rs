//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta577<F: Float>(t14693: F, t25270: F, t14927: F, t27261: F, t10778: F, t1941: F, t50538: F, t25222: F, t4435: F, t14868: F, t2661: F, t93082: F, t14751: F, t7045: F, t14757: F, t25234: F, t14738: F, t7038: F, t14732: F, t25245: F, t14668: F, t14933: F, t2482: F, t25260: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99054, t99056, t99063, t99066, t99069) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926::<F>(t14693, t25270, t14927, t27261, t10778, t1941, t50538, t25222, t4435, t14868, t2661, t93082);
        let (t99071, t99073, t99075, t99077, t99081, t99085) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1927::<F>(t14751, t7045, t14757, t25234, t14738, t7038, t14732, t25245, t14668, t27261, t14933, t2482, t25260, t814);
    (t99054, t99056, t99063, t99066, t99069, t99071, t99073, t99075, t99077, t99081, t99085)
}
