//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1145/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1145<F: Float>(t22914: F, t29582: F, t22953: F, t25893: F, t29692: F, t379: F, t101860: F, t3052: F, t6495: F, t25955: F, t3204: F, t22958: F, t5674: F, t22952: F, t26016: F, t925: F) -> (F, F, F, F, F, F) {
    let t116250 = t22914 * t29582;
    let t116254 = t25893 * t22953 * t29692 * t379;
    let t116258 = t101860 * t22953 * t6495 * t3052;
    let t116260 = t25955 * t3204;
    let t116262 = t5674 * t22958 * t116260;
    let t116266 = t22952 * t22953 * t26016 * t925;
    (t116250, t116254, t116258, t116260, t116262, t116266)
}
