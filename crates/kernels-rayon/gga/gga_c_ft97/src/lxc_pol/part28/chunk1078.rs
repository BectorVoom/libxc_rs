//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1078/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1078(t34674: f64, t8392: f64, t34700: f64, t1851: f64, t7165: f64, t11490: f64, t11593: f64, t11810: f64, t11863: f64, t1326: f64, t137729: f64, t137882: f64, t138307: f64, t144849: f64, t1901: f64, t1902: f64, t23249: f64, t23323: f64, t23327: f64, t23339: f64, t25590: f64, t26166: f64, t26171: f64, t26188: f64, t26245: f64, t26268: f64, t26371: f64, t26372: f64, t26373: f64, t26374: f64, t26390: f64, t26423: f64, t3113: f64, t3214: f64, t3219: f64, t32350: f64, t34647: f64, t34685: f64, t34706: f64, t379: f64, t38651: f64, t39150: f64, t7274: f64, t8372: f64, t8557: f64, t925: f64) -> f64 {
    let t145992 = t8392 * t34674;
    let t146020 = t8392 * t34700;
    let t146048 = t1851 * t7165;
    let t146061 = -2.0_f64 / 27.0_f64 * t145992 - 2.0_f64 / 9.0_f64 * t1901 * t11863 * t144849 - t1901 * t8557 * t32350 * t3113 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t23323 * t26268 + 4.0_f64 / 9.0_f64 * t11593 * t23327 * t26188 + 2.0_f64 / 27.0_f64 * t137729 - 2.0_f64 / 9.0_f64 * t1901 * t39150 * t34685 + 8.0_f64 * t1901 * t26372 * t38651 * t7274 * t3219 - 4.0_f64 * t1901 * t26372 * t26373 * t25590 - 2.0_f64 / 27.0_f64 * t146020 - 2.0_f64 / 9.0_f64 * t1901 * t8557 * t34647 * t379 + t1901 * t8372 * t34706 / 9.0_f64 + t1901 * t1902 * t137882 * t925 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t23323 * t26245 - 4.0_f64 * t1901 * t26371 * t1326 * t26374 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t26166 * t25590 + 2.0_f64 * t1901 * t26171 * t138307 * t3214 + 4.0_f64 / 3.0_f64 * t1901 * t11810 * t146048 * t3219 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t23339 * t26423 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t23249 * t26390;
    t146061
}
