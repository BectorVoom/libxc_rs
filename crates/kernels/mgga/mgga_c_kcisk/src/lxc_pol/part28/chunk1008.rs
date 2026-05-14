//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1008/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1008<F: Float>(t23322: F, t7261: F, t1772: F, t8793: F, t2448: F, t7218: F, t7208: F, t7230: F, t17165: F, t17169: F, t17172: F, t17184: F, t17187: F, t1773: F, t1778: F, t2460: F, t7219: F, t7248: F) -> (F, F, F) {
    let t23323 = t7261 * t23322;
    let t23326 = t8793 * t1772;
    let t23338 = t2448 * t7218;
    let t23341 = t7208 * t7230;
    let t23343 = 0.21588946459412780656e0 * t1773 * t23323 + 0.17990788716177317213e-1 * t23326 * t1778 - 0.71963154864709268852e-1 * t7208 * t7248 - 0.2398771828823642295e-1 * t17165 - 0.31983624384315230601e-1 * t17172 + 0.71963154864709268852e-1 * t17184 + t17187 - 0.95950873152945691804e-1 * t17169 * t2460 + 0.19190174630589138361e0 * t7219 * t7248 - 0.95950873152945691807e-1 * t23338 * t1778 + 0.11993859144118211475e-1 * t23341;
    (t23326, t23338, t23343)
}
