//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 906/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk906<F: Float>(t7335: F, t5522: F, t5525: F, t5745: F, t7352: F, t7357: F, t228: F, t5758: F, t261: F, t5812: F, t1918: F, t1957: F, t1972: F, t248: F, t2829: F, t704: F, t714: F, t723: F, t7446: F, t7447: F, t7475: F, t7478: F, t7485: F, t7486: F, t7491: F, t7493: F, t7494: F) -> (F, F, F, F, F, F) {
    let t7500 = 0.35616666666666666666e-1 * t7335;
    let t7502 = -t5745 + 0.47488888888888888888e-1 * t5522 - 0.17808333333333333333e-1 * t5525 + 0.23744444444444444444e-1 * t7357 - t7500 + 0.53425e-1 * t7352;
    let t7504 = 0.621814e-1 * t7502 * t228;
    let t7508 = 0.18541666666666666667e-1 * t7335;
    let t7510 = -t5758 + 0.24722222222222222222e-1 * t5522 - 0.92708333333333333333e-2 * t5525 + 0.12361111111111111111e-1 * t7357 - t7508 + 0.278125e-1 * t7352;
    let t7511 = t7510 * t261;
    let t7516 = 0.34246666666666666666e-1 * t7335;
    let t7518 = -t5812 + 0.45662222222222222222e-1 * t5522 - 0.17123333333333333333e-1 * t5525 + 0.22831111111111111111e-1 * t7357 - t7516 + 0.5137e-1 * t7352;
    let t7521 = -t7446 + 2.0 * t7447 * t704 + 0.5848223622634646207e0 * t714 * t7475 + 0.11696447245269292414e1 * t7478 * t723 + 0.5848223622634646207e0 * t2829 * t1972 + t7485 - 2.0 * t7486 * t1918 - t7491 - t7493 - 0.11696447245269292414e1 * t7494 * t1957 + t7504 - 0.19751673498613801407e-1 * t7511 - 0.310907e-1 * t7518 * t248;
    (t7502, t7504, t7510, t7511, t7518, t7521)
}
