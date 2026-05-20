//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1839;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta533<F: Float>(t4021: F, t94497: F, t2482: F, t25981: F, t27: F, t550: F, t7021: F, t25273: F, t540: F, t1372: F, t2019: F, t9951: F, t2018: F, t9646: F, t9723: F, t26014: F, t2689: F, t3994: F, t7028: F, t9845: F, t25240: F, t3951: F, t3964: F, t2681: F, t7269: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94498, t94508, t94513, t94519, t94520, t94522) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1839::<F>(t4021, t94497, t2482, t25981, t27, t550, t7021, t25273, t540, t1372, t2019, t9951);
        let (t94525, t94527, t94537, t94540, t94545) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1840::<F>(t2018, t9646, t9723, t26014, t2689, t3994, t7028, t9845, t25240, t3951, t3964, t2681, t7269, t820);
    (t94498, t94508, t94513, t94519, t94520, t94522, t94525, t94527, t94537, t94540, t94545)
}
