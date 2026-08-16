//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2205/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205(t100: f64, t9398: f64, t2341: f64, t657: f64, t12774: f64, t12775: f64, t12778: f64, t12795: f64, t1447: f64, t2219: f64, t2248: f64, t2336: f64, t2342: f64, t2350: f64, t2354: f64, t30171: f64, t30307: f64, t4049: f64, t4050: f64, t4054: f64, t45697: f64, t659: f64, t662: f64, t92: f64, t9212: f64, t9393: f64, t9404: f64) -> f64 {
    let t45707 = t100 * t9398;
    let t45717 = t657 * t2341;
    let t45731 = -10.0_f64 * t12774 * t9212 * t659 + 10.0_f64 * t12795 * t9212 * t662 - 10.0_f64 / 9.0_f64 * t45697 * t30171 * t2248 - 10.0_f64 / 9.0_f64 * t45697 * t2219 * t2342 + 10.0_f64 / 3.0_f64 * t12774 * t2219 * t2248 - 10.0_f64 / 9.0_f64 * t45707 * t30307 * t2354 + 10.0_f64 / 9.0_f64 * t45707 * t2219 * t2350 - 10.0_f64 / 3.0_f64 * t12795 * t2219 * t2354 - 100.0_f64 / 9.0_f64 * t45717 * t12775 + 400.0_f64 / 27.0_f64 * t2336 * t4050 + 200.0_f64 / 9.0_f64 * t2336 * t4054 - 50.0_f64 / 9.0_f64 * t657 * t12778 + 10.0_f64 / 9.0_f64 * t92 * t4049 * t9393 - 50.0_f64 / 9.0_f64 * t1447 * t9404;
    t45731
}
