//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 959/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk959<F: Float>(t13485: F, t8036: F, t3935: F, t1322: F, t7744: F, t3937: F, t3973: F, t8040: F, t1309: F, t13482: F, t13493: F, t20072: F, t20075: F, t20088: F, t20126: F, t20128: F, t26030: F, t26037: F, t26042: F, t26046: F, t26050: F, t26054: F, t26057: F, t26060: F, t3970: F, t8037: F, t8041: F, t8045: F) -> (F,) {
    let t26064 = t13485 * t8036;
    let t26065 = t3935 * t26064;
    let t26067 = t7744 * t1322;
    let t26068 = t3937 * t26067;
    let t26074 = t3973 * t8040;
    let t26075 = t1309 * t26074;
    let t26079 = -0.35981577432354634426e-1 * t13493 * t8037 - 0.35981577432354634426e-1 * t3935 * t26030 + 0.95950873152945691803e-1 * t13482 * t8037 + 0.71963154864709268852e-1 * t3935 * t26037 + 0.71963154864709268852e-1 * t3935 * t26042 - 0.47975436576472845901e-1 * t3935 * t26046 - 0.71963154864709268852e-1 * t3935 * t26050 + 0.35981577432354634426e-1 * t3935 * t26054 + 0.10794473229706390328e0 * t3935 * t26057 - 0.1439263097294185377e0 * t3935 * t26060 - 0.319836243843152306e-1 * t20072 + t20075 + t20088 - 0.11993859144118211475e-1 * t26065 - 0.17990788716177317213e-1 * t3935 * t26068 - t20126 - 0.799590609607880765e-2 * t20128 + 0.95950873152945691803e-1 * t3970 * t8041 - 0.11993859144118211475e-1 * t26075 - 0.47975436576472845901e-1 * t3970 * t8045;
    (t26079,)
}
