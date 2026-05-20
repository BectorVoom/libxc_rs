//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk998;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk999;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta166<F: Float>(t300: F, t4715: F, t4683: F, t1626: F, t983: F, t1642: F, t3022: F, t1633: F, t2986: F, t974: F, t981: F, t4707: F, t964: F, t973: F, t3011: F, t3014: F, t972: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4716, t4718, t4719) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk998::<F>(t300, t4715, t4683, t1626);
        let (t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk999::<F>(t4719, t983, t1642, t3022, t1633, t2986, t974, t981, t4707, t964, t973, t3011);
        let t4733 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1000::<F>(t3014, t972);
    (t4716, t4718, t4719, t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732, t4733)
}
