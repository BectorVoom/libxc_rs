//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 814/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk814<F: Float>(t12313: F, t2102: F, t12317: F, t9217: F, t11050: F, t3506: F, t11059: F, t3499: F, t1017: F, t2112: F, t1970: F, t1570: F, t586: F) -> (F, F, F, F, F, F) {
    let t12775 = t2102 * t12313;
    let t12778 = t9217 * t12317;
    let t12781 = t3506 * t11050;
    let t12784 = t3499 * t11059;
    let t12787 = t2112 * t1017;
    let t12788 = t12787 * t1970;
    let t12791 = t586 * t1570;
    (t12775, t12778, t12781, t12784, t12788, t12791)
}
