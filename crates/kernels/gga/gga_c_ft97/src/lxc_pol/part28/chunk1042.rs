//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1042/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1042<F: Float>(t136759: F, t373: F, t53: F, t930: F, t1593: F, t1609: F, t3037: F, t58: F, t136815: F, t3066: F, t938: F, t136825: F, t32169: F, t34472: F) -> (F, F, F, F) {
    let t145200 = t136759 * t373 * t930 * t53;
    let t145205 = t1609 * t58 * t1593 * t3037;
    let t145209 = t136815 * t938 * t3066;
    let t145223 = t32169 * t136825 * t34472;
    (t145200, t145205, t145209, t145223)
}
