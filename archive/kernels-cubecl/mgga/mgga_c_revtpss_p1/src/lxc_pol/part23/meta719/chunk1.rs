//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2479/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2479<F: Float>(t1389: F, t1882: F, t46856: F, t543: F, t685: F, t72: F, t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t46917: F, t5701: F) -> (F, F, F, F) {
    let t48563 = t46856 * t1389 * t1882 * t543 * t72 * t685;
    let t48600 = t46946 * t13955;
    let t48603 = t9845 * t808 * t13775;
    let t48604 = F::cast_from(0.76230004213927992336e-5_f64) * t48603;
    let t48614 = t46917 * t5701;
    (t48563, t48600, t48604, t48614)
}
