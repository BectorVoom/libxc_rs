//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk563;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk564;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk565;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk566;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk567;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk568;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk569;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk570;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk571;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta93<F: Float>(t2018: F, t213: F, t552: F, t1955: F, t555: F, t1032: F, t1426: F, t68: F, t72: F, t1927: F, t5: F, t1923: F, t117: F, t114: F, t1934: F, t508: F, t1943: F, t1947: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2019, t2020, t2027) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk563::<F>(t2018, t213, t552, t1955, t555);
        let t2028 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk564::<F>(t1032, t1426);
        let t2047 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk565::<F>(t68, t72);
        let t2048 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk566::<F>(t1927, t2047);
        let t2051 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk567::<F>(t5, t1923, t2048);
        let t2052 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk568::<F>(t117, t2051);
        let t2055 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk569::<F>(t114, t1934);
        let t2056 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk570::<F>(t2055, t508);
        let t2061 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk571::<F>(t1943, t1947);
        let t2062 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk572::<F>(t2061, t225);
    (t2019, t2020, t2027, t2028, t2047, t2048, t2051, t2052, t2055, t2056, t2061, t2062)
}
