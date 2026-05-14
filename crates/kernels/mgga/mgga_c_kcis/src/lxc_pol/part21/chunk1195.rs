//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1195/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1195<F: Float>(t1851: F, t30045: F, t3620: F, t5329: F, t15171: F, t5310: F, t922: F, t7794: F, t993: F, t2888: F, t27028: F, t15256: F, t15481: F, t15501: F, t26955: F, t26960: F, t7772: F, t92851: F, t95928: F, t95931: F, t95946: F, t96799: F, t96995: F, t96999: F) -> (F, F, F) {
    let t97069 = t5329 * t30045 * t1851 * t3620;
    let t97076 = t5310 * t15171 * t922;
    let t97083 = t993 * t7794;
    let t97089 = t2888 * t7794;
    let t97093 = t993 * t27028;
    let t97098 = 0.13913205078125e-3 * t7772 * t97069 + 0.10306077835648148148e-4 * t92851 - 0.30952962962962962962e-2 * t95928 + 0.25794135802469135802e-2 * t95931 + 0.30918233506944444444e-4 * t26955 * t97076 + 0.15459116753472222222e-4 * t26955 * t96995 + 0.20612155671296296296e-4 * t26955 * t96999 - 0.46336805555555555556e-3 * t26960 * t97083 * t15481 - 0.30918233506944444444e-4 * t26955 * t96799 + 0.30891203703703703704e-3 * t26960 * t97089 * t15256 - 0.46336805555555555556e-3 * t26960 * t97093 * t15501 + 0.12897067901234567901e-2 * t95946;
    (t97069, t97076, t97098)
}
