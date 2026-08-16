//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2561/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561(t18683: f64, t51249: f64, t14850: f64, t18262: f64, t18266: f64, t51120: f64, t1117: f64, t11275: f64, t21961: f64, t11190: f64, t4781: f64, t6024: f64) -> (f64, f64, f64, f64, f64) {
    let t71793 = 0.2894756309764656312e3_f64 * t51249 * t18683;
    let t71795 = 0.96491876992155210402e2_f64 * t14850 * t18262;
    let t71797 = 0.1551780387578202009e4_f64 * t51120 * t18266;
    let t71800 = 0.57895126195293126241e3_f64 * t11275 * t21961 * t1117;
    let t71803 = 0.28947563097646563121e3_f64 * t11190 * t6024 * t4781;
    (t71793, t71795, t71797, t71800, t71803)
}
