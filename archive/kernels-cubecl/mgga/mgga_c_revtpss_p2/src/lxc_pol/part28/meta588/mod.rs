//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2056;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta588<F: Float>(t25273: F, t540: F, t1372: F, t2019: F, t9951: F, t2018: F, t9646: F, t9723: F, t26014: F, t2689: F, t807: F, t9714: F, t9703: F, t3994: F, t7028: F, t9845: F, t25240: F, t3951: F, t3964: F, t25972: F, t9761: F, t2681: F, t7269: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94519, t94520, t94523, t94526, t94527, t94530) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2056::<F>(t25273, t540, t1372, t2019, t9951, t2018, t9646, t9723, t26014, t2689, t807, t9714);
        let (t94534, t94537, t94540, t94542, t94545) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2057::<F>(t2018, t807, t9703, t3994, t7028, t9845, t25240, t3951, t3964, t25972, t9761, t2681, t7269, t820);
    (t94519, t94520, t94523, t94526, t94527, t94530, t94534, t94537, t94540, t94542, t94545)
}
