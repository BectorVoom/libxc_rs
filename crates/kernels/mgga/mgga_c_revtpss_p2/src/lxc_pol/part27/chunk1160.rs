//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1160/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1160<F: Float>(t11239: F, t487: F, t1276: F, t2148: F, t2142: F, t3596: F, t3601: F, t3769: F, t3783: F, t7660: F, t1269: F, t3140: F) -> (F, F, F, F, F) {
    let t26904 = t487 * t11239;
    let t26906 = t2148 * t26904 * t1276;
    let t26907 = t3596 * t2142;
    let t26909 = t26907 * t3601 * t3769;
    let t26913 = t7660 * t3601 * t3783;
    let t26916 = t1269 * t3140;
    (t26906, t26907, t26909, t26913, t26916)
}
