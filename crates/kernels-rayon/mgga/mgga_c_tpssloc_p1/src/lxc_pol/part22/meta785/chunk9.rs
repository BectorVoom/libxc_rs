//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2712/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712(t1449: f64, t5484: f64, t100: f64, t103: f64, t12795: f64, t1447: f64, t19514: f64, t19518: f64, t19522: f64, t19525: f64, t19526: f64, t2: f64, t20331: f64, t20338: f64, t2219: f64, t2349: f64, t4059: f64, t4064: f64, t45460: f64, t45707: f64, t5475: f64, t5480: f64, t55491: f64, t584: f64, t662: f64, t75649: f64) -> f64 {
    let t75676 = t1449 * t5484;
    let t75694 = -200.0_f64 / 9.0_f64 * t5475 * t4064 + 50.0_f64 / 27.0_f64 * t1447 * t19514 + 100.0_f64 / 9.0_f64 * t55491 * t19518 - 50.0_f64 / 9.0_f64 * t1447 * t19522 - 25.0_f64 / 3.0_f64 * t1447 * t19526 + 40.0_f64 / 81.0_f64 * t100 * t45460 * t20331 * t662 + 10.0_f64 / 9.0_f64 * t45707 * t5480 * t2 * t584 - 10.0_f64 / 9.0_f64 * t45707 * t75676 * t662 - 10.0_f64 / 3.0_f64 * t12795 * t2219 * t5484 + 10.0_f64 / 3.0_f64 * t100 * t4059 * t19525 + 10.0_f64 / 9.0_f64 * t100 * t2349 * t20338 * t662 - 5.0_f64 / 3.0_f64 * t100 * t103 * t75649;
    t75694
}
