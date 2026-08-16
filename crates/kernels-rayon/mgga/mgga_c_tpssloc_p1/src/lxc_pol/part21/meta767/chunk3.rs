//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2648/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648(t1447: f64, t2349: f64, t100: f64, t12792: f64, t12796: f64, t12799: f64, t12805: f64, t19493: f64, t19498: f64, t19513: f64, t19521: f64, t19525: f64, t21: f64, t2248: f64, t2341: f64, t2342: f64, t2350: f64, t2354: f64, t4049: f64, t4059: f64, t45460: f64, t45496: f64, t45717: f64, t5396: f64, t5468: f64, t5480: f64, t5484: f64, t584: f64, t662: f64, t9: f64, t92: f64, t9384: f64, t9398: f64) -> f64 {
    let t55491 = t1447 * t2349;
    let t55512 = 10.0_f64 / 9.0_f64 * t92 * t19498 * t2248 - 100.0_f64 / 27.0_f64 * t1447 * t12799 - 10.0_f64 / 27.0_f64 * t100 * t19513 * t2354 + 20.0_f64 / 9.0_f64 * t100 * t2349 * t9 * t21 + 40.0_f64 / 81.0_f64 * t92 * t45496 * t5468 * t2342 + 20.0_f64 / 9.0_f64 * t92 * t4049 * t584 - 10.0_f64 / 27.0_f64 * t92 * t9384 * t5396 * t2342 + 100.0_f64 / 81.0_f64 * t1447 * t12792 - 50.0_f64 / 3.0_f64 * t1447 * t12805 + 40.0_f64 / 81.0_f64 * t100 * t45460 * t5480 * t2350 - 200.0_f64 / 27.0_f64 * t45717 * t19493 + 200.0_f64 / 27.0_f64 * t55491 * t12796 - 20.0_f64 / 9.0_f64 * t100 * t4059 * t584 - 10.0_f64 / 27.0_f64 * t100 * t9398 * t5484 * t2350 + 20.0_f64 / 9.0_f64 * t92 * t2341 * t9 * t21 + 20.0_f64 / 9.0_f64 * t100 * t2349 * t19525 * t662 + 10.0_f64 / 9.0_f64 * t100 * t19521 * t2354;
    t55512
}
