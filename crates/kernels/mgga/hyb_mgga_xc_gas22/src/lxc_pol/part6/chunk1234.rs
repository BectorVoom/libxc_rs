//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1234/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1234<F: Float>(t20643: F, t2195: F, t4117: F, t4121: F, t6601: F, t20631: F, t10566: F, t2200: F, t3324: F, t8682: F, t10577: F, t2194: F, t791: F, t10571: F, t10584: F, t3335: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28877 = t20643 * t4117 * t2195;
    let t28880 = t6601 * t4121 * t2195;
    let t28883 = t20631 * t4117 * t2195;
    let t28885 = t10566 * t2200;
    let t28887 = t3324 * t8682;
    let t28890 = t2194 * t10577 * t791;
    let t28892 = t10571 * t2200;
    let t28894 = t10584 * t2200;
    let t28896 = t3335 * t8682;
    (t28877, t28880, t28883, t28885, t28887, t28890, t28892, t28894, t28896)
}
