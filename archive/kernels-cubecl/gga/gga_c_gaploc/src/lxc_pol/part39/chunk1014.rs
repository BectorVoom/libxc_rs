//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1014/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1014<F: Float>(t43400: F, t33308: F, t9805: F, t9806: F, t15499: F, t28640: F, t3487: F, t40966: F, t2963: F, t3295: F, t9796: F, t40969: F) -> (F, F, F, F, F, F) {
    let t43401 = F::cast_from(0.15337170381568299871e1_f64) * t43400;
    let t43403 = t9805 * t33308 * t9806;
    let t43404 = F::cast_from(0.10352590007558602413e2_f64) * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = F::cast_from(0.46011511144704899612e1_f64) * t43407;
    let t43409 = F::cast_from(0.11502877786176224903e1_f64) * t40966;
    let t43412 = t9796 * t2963 * t3295;
    let t43413 = F::cast_from(0.76685851907841499353e0_f64) * t43412;
    let t43414 = F::cast_from(0.38342925953920749676e1_f64) * t40969;
    (t43401, t43404, t43408, t43409, t43413, t43414)
}
