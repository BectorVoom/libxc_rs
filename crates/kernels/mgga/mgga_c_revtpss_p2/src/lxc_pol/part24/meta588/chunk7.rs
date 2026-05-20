//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1843/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1843<F: Float>(t47067: F, t47070: F, t47072: F, t47074: F, t47076: F, t91970: F, t91974: F, t91975: F, t91976: F, t91977: F, t91978: F, t91979: F, t91980: F, t91982: F, t91983: F) -> F {
    let t92469 = -t91970 + t91974 + t47067 - t91975 + t47070 - t47072 - t47074 - t91976 - t91977 - t47076 + t91978 - t91979 - t91980 - t91982 - t91983;
    t92469
}
