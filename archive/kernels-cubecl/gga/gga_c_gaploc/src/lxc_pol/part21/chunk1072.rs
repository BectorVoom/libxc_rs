//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1072/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1072<F: Float>(t1321: F, t481: F, t880: F, t169: F, t18310: F, t1381: F, t2353: F, t501: F, t8040: F, t1959: F, t2967: F, t747: F, t9032: F) -> (F, F, F, F, F, F) {
    let t23983 = t481 * t880 * t1321;
    let t24139 = t18310 * t169;
    let t24215 = t2353 * t1381;
    let t24282 = t8040 * t501;
    let t24295 = t2967 * t1959;
    let t24303 = t9032 * t747;
    (t23983, t24139, t24215, t24282, t24295, t24303)
}
