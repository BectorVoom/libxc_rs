//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1018/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1018(t20395: f64, t3238: f64, t11556: f64, t1901: f64, t1909: f64, t20218: f64, t20471: f64, t2983: f64, t4417: f64, t446: f64, t452: f64, t4589: f64, t60756: f64, t74959: f64, t75584: f64, t75586: f64, t75588: f64, t75590: f64, t75624: f64, t75642: f64, t8424: f64, t925: f64, t942: f64) -> (f64, f64) {
    let t85928 = t3238 * t20395;
    let t85988 = -4.0_f64 / 3.0_f64 * t446 * t452 * t20471 * t942 - 4.0_f64 / 9.0_f64 * t75584 - 8.0_f64 / 9.0_f64 * t75586 + 8.0_f64 / 27.0_f64 * t75588 + 8.0_f64 / 9.0_f64 * t75590 - 4.0_f64 / 3.0_f64 * t1901 * t1909 * t8424 * t4417 * t4589 + 4.0_f64 / 9.0_f64 * t1901 * t1909 * t74959 * t925 + 8.0_f64 / 9.0_f64 * t75624 + 4.0_f64 / 3.0_f64 * t75642 + 8.0_f64 / 9.0_f64 * t1901 * t11556 * t2983 * t20218 + 16.0_f64 / 27.0_f64 * t60756;
    (t85928, t85988)
}
