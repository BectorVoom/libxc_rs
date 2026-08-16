//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 962/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk962(t1888: f64, t31333: f64, t82159: f64, t23012: f64, t8548: f64, t214: f64, t7084: f64, t6552: f64, t6555: f64, t10049: f64, t10110: f64, t112863: f64, t112868: f64, t112872: f64, t112877: f64, t112881: f64, t2054: f64, t24297: f64, t25168: f64, t2597: f64, t2718: f64, t2719: f64, t2743: f64, t31399: f64, t31409: f64, t31423: f64, t6631: f64, t6663: f64, t82197: f64, t855: f64, t8553: f64, t8562: f64, t8563: f64, t865: f64, t92981: f64, t9590: f64) -> (f64, f64) {
    let t114842 = t1888 * t82159 * t31333;
    let t114864 = t23012 * t8548;
    let t114865 = 0.63969658155208805863e-1_f64 * t114864;
    let t114866 = t214 * t7084;
    let t114868 = t6552 * t114866 * t6555;
    let t114870 = 0.3289868133696452873e-1_f64 * t114842 + 4.0_f64 * t2597 * t31409 + 2.0_f64 * t9590 * t8553 + t112863 - t82197 * t2054 - t112868 + t112872 - 6.0_f64 * t855 * t10110 * t8562 * t2719 - 12.0_f64 * t25168 * t92981 * t6631 - t31423 * t2743 - t10049 * t8563 - 2.0_f64 * t24297 * t6663 + t112877 - t112881 + 4.0_f64 * t855 * t2718 * t31399 * t865 - t114865 - 0.3289868133696452873e-1_f64 * t114868;
    (t114866, t114870)
}
