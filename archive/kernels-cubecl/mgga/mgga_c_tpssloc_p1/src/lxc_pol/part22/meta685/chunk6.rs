//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2262/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262<F: Float>(t3375: F, t6063: F, t18893: F, t3359: F, t11285: F, t6084: F, t18785: F, t3403: F, t18834: F, t3315: F, t1147: F, t18710: F) -> (F, F, F, F, F, F) {
    let t63454 = t6063 * t3375;
    let t63502 = t18893 * t3359;
    let t63519 = t6084 * t11285;
    let t63533 = t18785 * t3403;
    let t63588 = t18834 * t3315;
    let t63597 = t18710 * t1147;
    (t63454, t63502, t63519, t63533, t63588, t63597)
}
