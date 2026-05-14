//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1090/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1090<F: Float>(t11612: F, t2300: F, t3723: F, t10373: F, t3724: F, t11619: F, t2493: F, t3209: F, t1054: F, t2316: F, t10105: F, t11620: F, t2255: F, t2674: F, t996: F, t2405: F) -> (F, F, F, F, F, F, F) {
    let t36020 = t11612 * t3723 * t2300;
    let t36022 = t10373 * t3724;
    let t36025 = t3209 * t11619 * t2493;
    let t36028 = t1054 * t3723 * t2316;
    let t36030 = t10105 * t11620;
    let t36034 = t996 * t2674 * t3723 * t2255;
    let t36037 = t1054 * t3723 * t2405;
    (t36020, t36022, t36025, t36028, t36030, t36034, t36037)
}
