//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk935;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk936;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta150<F: Float>(t4305: F, t706: F, t190: F, t4186: F, t1531: F, t705: F, t707: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F, t1522: F, t2398: F, t1568: F, t212: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4306, t4307, t4308, t4310, t4311) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk935::<F>(t4305, t706, t190, t4186, t1531, t705);
        let (t4313, t4314) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk936::<F>(t4311, t707, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304, t4307, t4310);
        let (t4316, t4321) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk937::<F>(t1522, t2398, t1568, t212);
    (t4306, t4307, t4308, t4310, t4311, t4313, t4314, t4316, t4321)
}
