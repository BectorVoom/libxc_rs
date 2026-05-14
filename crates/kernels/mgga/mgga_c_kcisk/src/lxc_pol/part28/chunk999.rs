//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 999/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk999<F: Float>(t1659: F, t22396: F, t1856: F, t22501: F, t1835: F, t22506: F, t22484: F, t22488: F, t1842: F, t11524: F, t158: F, t16204: F, t16206: F, t16208: F, t16211: F, t16217: F, t16223: F, t16225: F, t16227: F, t16229: F, t16232: F, t165: F, t173: F, t5816: F, t5827: F) -> (F,) {
    let t23177 = t1659 * t22396;
    let t23180 = t1856 * t22501;
    let t23183 = t1835 * t22506;
    let t23192 = t1835 * t22484;
    let t23195 = t1835 * t22488;
    let t23198 = t1842 * t22484;
    let t23201 = t1842 * t22488;
    let t23208 = 0.21133333333333333333e-2 * t5816 * t23177 - 0.10082625e-4 * t173 * t23180 - 0.672175e-5 * t173 * t23183 + 0.70444444444444444443e-2 * t16204 + 0.78420416666666666667e-4 * t16206 - 0.21858666666666666667e-1 * t16208 - 0.18736e-1 * t16211 + 0.52833333333333333332e-2 * t16217 + 0.4705225e-4 * t16223 - 0.21078e-1 * t158 * t23192 - 0.28104e-1 * t5827 * t23195 + 0.4755e-2 * t165 * t23198 + 0.634e-2 * t5816 * t23201 + 0.47822877300252710492e-1 * t16225 - 0.62154466893555682512e-3 * t16227 + 0.47822877300252710492e-1 * t16229 - 0.41436311262370455008e-3 * t16232 - t11524;
    (t23208,)
}
