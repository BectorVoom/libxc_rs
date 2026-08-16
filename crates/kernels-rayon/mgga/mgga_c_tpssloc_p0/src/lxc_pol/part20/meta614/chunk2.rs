//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2206/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206(t100: f64, t4063: f64, t591: f64, t4053: f64, t92: f64, t103: f64, t12771: f64, t12781: f64, t12784: f64, t1444: f64, t1445: f64, t1447: f64, t1449: f64, t16: f64, t2341: f64, t2349: f64, t4059: f64, t45460: f64, t45496: f64, t584: f64, t657: f64, t659: f64, t662: f64, t9374: f64, t9385: f64, t9399: f64, t9400: f64, t9407: f64, t9408: f64, t95: f64) -> f64 {
    let t45751 = 20.0_f64 * t100 * t4063 * t591;
    let t45762 = 20.0_f64 * t92 * t4053 * t591;
    let t45775 = 10.0_f64 / 9.0_f64 * t100 * t4059 * t9407 - 2200.0_f64 / 81.0_f64 * t9374 * t1445 - 25.0_f64 / 3.0_f64 * t657 * t12781 - 10.0_f64 * t92 * t95 * t16 + 50.0_f64 / 81.0_f64 * t1447 * t9400 + 10.0_f64 * t100 * t103 * t16 - 25.0_f64 / 9.0_f64 * t1447 * t9408 - t45751 + 40.0_f64 / 81.0_f64 * t92 * t45496 * t1444 * t9385 + 10.0_f64 / 3.0_f64 * t92 * t2341 * t584 * t659 + t45762 + 40.0_f64 / 81.0_f64 * t100 * t45460 * t1449 * t9399 - 10.0_f64 / 3.0_f64 * t100 * t2349 * t584 * t662 + 50.0_f64 / 27.0_f64 * t657 * t12771 + 25.0_f64 * t657 * t12784;
    t45775
}
