//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 718/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk718<F: Float>(t5967: F, t9068: F, t1510: F, t1720: F, t3108: F, t1043: F, t1668: F, t3017: F, t5022: F, t3157: F, t8948: F, t1645: F, t190: F, t1649: F, t1643: F, t9135: F, t9138: F, t9140: F, t9142: F, t9145: F, t9148: F, t9151: F) -> (F, F, F) {
    let t9153 = t9068 * t5967;
    let t9155 = t1720 * t1510;
    let t9156 = t3108 * t9155;
    let t9158 = t1043 * t1668;
    let t9160 = t3017 * t5022;
    let t9161 = t1043 * t9160;
    let t9163 = t8948 * t3157;
    let t9166 = t190 * t1645 * M_PI;
    let t9167 = t9166 * t1649;
    let t9168 = t1643 * t9167;
    let t9170 = 0.13900948042322754167e-2 * t9135 + 0.10120768229166666667e-4 * t9138 - 0.6487109086417285278e-2 * t9140 + 0.1374296967252737644e-5 * t9142 - 0.38647271295071362318e-6 * t9145 + 0.687148483626368822e-6 * t9148 - 0.21135226489492151266e-6 * t9151 + 0.42270452978984302532e-6 * t9153 + 0.27801896084645508334e-2 * t9156 + 0.33816362383187442026e-4 * t9158 + 0.43478180206955282604e-5 * t9161 - 0.19679271556712962963e-4 * t9163 + 0.38010404803226280926e-3 * t9168;
    (t9160, t9166, t9170)
}
