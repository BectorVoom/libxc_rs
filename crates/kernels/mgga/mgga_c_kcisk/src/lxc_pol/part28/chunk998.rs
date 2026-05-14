//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 998/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk998<F: Float>(t1835: F, t22396: F, t22501: F, t22506: F, t706: F, t22387: F, t1919: F, t22392: F, t1842: F, t1659: F, t4726: F, t158: F, t16163: F, t16167: F, t16188: F, t16190: F, t165: F, t173: F, t23134: F, t23137: F, t23140: F, t23143: F, t5823: F, t5827: F) -> (F,) {
    let t23146 = t1835 * t22396;
    let t23149 = t1835 * t22501;
    let t23152 = t706 * t22506;
    let t23155 = t706 * t22387;
    let t23158 = t1919 * t22392;
    let t23161 = t706 * t22396;
    let t23164 = t1842 * t22501;
    let t23167 = t1659 * t22506;
    let t23170 = t1659 * t22387;
    let t23173 = t4726 * t22392;
    let t23176 = t16163 + 0.31368166666666666667e-4 * t16167 - t16188 - 0.31226666666666666667e-2 * t16190 + 0.30247875e-4 * t173 * t23134 + 0.403305e-4 * t173 * t23137 + 0.403305e-4 * t5823 * t23140 + 0.22405833333333333333e-5 * t173 * t23143 + 0.26887e-4 * t5823 * t23146 + 0.7026e-2 * t158 * t23149 + 0.1171e-2 * t158 * t23152 - 0.7026e-2 * t158 * t23155 + 0.78066666666666666667e-3 * t158 * t23158 - 0.4684e-2 * t5827 * t23161 - 0.1585e-2 * t165 * t23164 - 0.52833333333333333333e-3 * t165 * t23167 + 0.317e-2 * t165 * t23170 - 0.17611111111111111111e-3 * t165 * t23173;
    (t23176,)
}
