//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1342;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta368(t40135: f64, t760: f64, t10565: f64, t606: f64, t706: f64, t10468: f64, t750: f64, t10555: f64, t10605: f64, t10436: f64, t2398: f64, t10356: f64, t10439: f64, t717: f64, t39989: f64, t40126: f64, t40128: f64, t40131: f64, t40133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40137, t40140, t40142, t40144, t40146, t40148) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1342(t40135, t760, t10565, t606, t706, t10468, t750, t10555, t10605, t10436, t2398, t10356, t10439);
        let (t40149, t40151, t40152) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1343(t40148, t10565, t717, t39989, t40126, t40128, t40131, t40133, t40137, t40140, t40142, t40144, t40146);
    (t40137, t40140, t40142, t40144, t40146, t40149, t40151, t40152)
}
