//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1122/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1122<F: Float>(t10366: F, t11613: F, t11616: F, t3209: F, t11682: F, t23678: F, t2415: F, t2546: F, t11612: F, t2300: F, t3723: F, t10373: F, t3724: F, t11619: F, t2493: F, t1054: F, t2316: F) -> (F, F, F, F, F, F, F) {
    let t36011 = t10366 * t11613;
    let t36013 = t3209 * t11616;
    let t36017 = t11682 * t2415 * t2546 * t23678;
    let t36020 = t11612 * t3723 * t2300;
    let t36022 = t10373 * t3724;
    let t36025 = t3209 * t11619 * t2493;
    let t36028 = t1054 * t3723 * t2316;
    (t36011, t36013, t36017, t36020, t36022, t36025, t36028)
}
