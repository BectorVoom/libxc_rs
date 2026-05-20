//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2014;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta590<F: Float>(t25240: F, t3951: F, t3964: F, t2681: F, t7269: F, t820: F, t1416: F, t240: F, t25981: F, t25987: F, t9775: F, t2453: F, t4086: F, t64: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t1445: F, t2439: F, t25916: F, t25877: F, t94390: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94540, t94545, t94546, t94550, t94554, t94564) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2014::<F>(t25240, t3951, t3964, t2681, t7269, t820, t1416, t240, t25981, t25987, t9775, t2453, t4086, t64);
        let (t94565, t94569, t94571, t94580, t94589) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2015::<F>(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t1445, t2439, t25916, t25877, t94390);
    (t94540, t94545, t94546, t94550, t94554, t94564, t94565, t94569, t94571, t94580, t94589)
}
