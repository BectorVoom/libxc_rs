//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 886/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk886<F: Float>(t42629: F, t42633: F, t42637: F, t42638: F, t42641: F, t42645: F, t42648: F, t42652: F, t42655: F, t42659: F, t42674: F, t46918: F, t46923: F, t46928: F, t46931: F, t46933: F, t46944: F, t46947: F) -> (F,) {
    let t50953 = -t46918 + t46923 + 0.56910013271352299198e-1 * t46928 + t46931 - t46933 - t42629 - t42633 + t42637 - t42638 + t42641 - t42645 + t42648 - t42652 + t42655 - t42659 - t42674 + t46944 + 0.1138200265427045984e0 * t46947;
    (t50953,)
}
