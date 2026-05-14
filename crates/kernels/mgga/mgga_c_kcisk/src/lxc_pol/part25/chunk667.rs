//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 667/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk667<F: Float>(t1725: F, t7134: F, t2417: F, t4911: F, t1724: F, t4838: F, t4915: F, t7076: F, t7079: F, t7082: F, t7086: F, t2422: F, t45: F, t2430: F, t4928: F, t1744: F, t1746: F) -> (F, F, F, F, F, F, F) {
    let t7135 = t7134 * t1725;
    let t7138 = t2417 * t4911;
    let t7139 = t7138 * t1724;
    let t7147 = t4915 + 0.30902777777777777778e-2 * t4838 + 0.30902777777777777778e-2 * t7076 - 0.61805555555555555555e-2 * t7079 + 0.18541666666666666667e-1 * t7082 + 0.18541666666666666667e-1 * t7086;
    let t7151 = t45 * t2422;
    let t7156 = t4928 * t2430;
    let t7157 = t1746 * t1744;
    (t7135, t7138, t7139, t7147, t7151, t7156, t7157)
}
