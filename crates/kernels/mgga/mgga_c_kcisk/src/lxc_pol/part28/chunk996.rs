//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 996/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk996<F: Float>(t1824: F, t23103: F, t11461: F, t11463: F, t11495: F, t11612: F, t16081: F, t16084: F, t1809: F, t23062: F, t23065: F, t23068: F, t23070: F, t23072: F, t23074: F, t23077: F, t23081: F, t23084: F, t23087: F, t23090: F, t23093: F, t23097: F, t23100: F, t5089: F, t5134: F, t674: F) -> (F,) {
    let t23104 = t23103 * t1824;
    let t23107 = -0.46853067927761790996e-2 * t11461 - 0.93706135855523581992e-2 * t11463 - 0.23426533963880895498e-2 * t1809 * t23062 - 0.46853067927761790996e-2 * t674 * t23065 + 0.46853067927761790996e-2 * t23068 - 0.46853067927761790996e-2 * t23070 + 0.23426533963880895498e-2 * t23072 - 0.14055920378328537299e-1 * t23074 - 0.93706135855523581992e-2 * t16081 + t16084 - 0.14055920378328537299e-1 * t11495 * t23077 - 0.93706135855523581992e-2 * t5089 * t23081 - 0.18741227171104716398e-1 * t11612 * t23084 + 0.93706135855523581992e-2 * t1809 * t23087 - 0.18741227171104716398e-1 * t5134 * t23090 + 0.46853067927761790996e-2 * t5089 * t23093 + 0.46853067927761790996e-2 * t1809 * t23097 - 0.14055920378328537299e-1 * t1809 * t23100 - 0.56223681513314149196e-1 * t674 * t23104;
    (t23107,)
}
