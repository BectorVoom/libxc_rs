//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2370/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370(t10375: f64, t1612: f64, t1041: f64, t1539: f64, t248: f64, t42749: f64, t14473: f64, t2952: f64, t10633: f64, t4483: f64, t47705: f64, t47707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    let t48679 = 0.51947577317044391276e2_f64 * t14473 * t2952;
    let t48681 = 0.10254018858216406658e4_f64 * t4483 * t10633;
    let t48688 = 0.47488888888888888888e-1_f64 * t47705;
    let t48689 = 0.15829629629629629629e-1_f64 * t47707;
    (t48670, t48674, t48679, t48681, t48688, t48689)
}
