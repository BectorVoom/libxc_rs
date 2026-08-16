//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1226/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1226(t1874: f64, t33690: f64, t7266: f64, t7461: f64, t27863: f64, t1459: f64, t1774: f64, t1869: f64, t1976: f64, t2114: f64, t2165: f64, t31880: f64, t32659: f64, t33686: f64, t33688: f64, t510: f64, t6517: f64, t7451: f64, t7670: f64, t7983: f64, t7989: f64, t8103: f64, t8667: f64) -> f64 {
    let t33691 = t33690 * t1874;
    let t33693 = t7266 * t7461;
    let t33697 = t27863 * t1874;
    let t33702 = -2.0_f64 * t1459 * t31880 - t1774 * t8667 - t1869 * t8103 - t1976 * t7983 - t2114 * t7670 - t2165 * t7451 - t33686 * t510 - 2.0_f64 * t6517 * t7989 - 2.0_f64 * t32659 - 2.0_f64 * t33688 - 2.0_f64 * t33691 - 2.0_f64 * t33693 - 2.0_f64 * t33697;
    t33702
}
