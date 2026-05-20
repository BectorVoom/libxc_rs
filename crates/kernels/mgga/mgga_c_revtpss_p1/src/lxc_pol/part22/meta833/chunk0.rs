//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2956/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2956<F: Float>(t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t46917: F, t5701: F, t14005: F, t46740: F, t5697: F, t1872: F, t4057: F, t9816: F, t9818: F) -> (F, F, F, F, F, F) {
    let t48600 = t46946 * t13955;
    let t48603 = t9845 * t808 * t13775;
    let t48614 = t46917 * t5701;
    let t48637 = t46740 * t14005;
    let t48645 = t46917 * t5697;
    let t48655 = t9816 * t9818 * t1872 * t4057;
    (t48600, t48603, t48614, t48637, t48645, t48655)
}
