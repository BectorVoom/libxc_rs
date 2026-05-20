//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1581;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1582;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1583;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1584;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1585;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1586;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1587;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta422<F: Float>(t2435: F, t3373: F, t3369: F, t12313: F, t689: F, t12319: F, t128: F, t3360: F, t43789: F, t1120: F, t43793: F, t43797: F, t43854: F, t43881: F, t43883: F, t43886: F, t43888: F) -> (F, F, F, F, F, F, F, F) {
        let t43890 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1581::<F>(t2435, t3373);
        let t43892 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1582::<F>(t2435, t3369);
        let t43894 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1583::<F>(t12313, t689);
        let t43896 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1584::<F>(t12319, t689);
        let t43899 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1585::<F>(t128, t3360, t43789);
        let t43902 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1586::<F>(t1120, t128, t43793);
        let t43905 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1587::<F>(t1120, t128, t43797);
        let t43907 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588::<F>(t43854, t43881, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
    (t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43907)
}
