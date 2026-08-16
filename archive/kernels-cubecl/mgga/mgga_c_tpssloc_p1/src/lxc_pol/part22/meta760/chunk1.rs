//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2561/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561<F: Float>(t18683: F, t51249: F, t14850: F, t18262: F, t18266: F, t51120: F, t1117: F, t11275: F, t21961: F, t11190: F, t4781: F, t6024: F) -> (F, F, F, F, F) {
    let t71793 = F::cast_from(0.2894756309764656312e3_f64) * t51249 * t18683;
    let t71795 = F::cast_from(0.96491876992155210402e2_f64) * t14850 * t18262;
    let t71797 = F::cast_from(0.1551780387578202009e4_f64) * t51120 * t18266;
    let t71800 = F::cast_from(0.57895126195293126241e3_f64) * t11275 * t21961 * t1117;
    let t71803 = F::cast_from(0.28947563097646563121e3_f64) * t11190 * t6024 * t4781;
    (t71793, t71795, t71797, t71800, t71803)
}
