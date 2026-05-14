//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 940/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk940<F: Float>(t34674: F, t8392: F, t34700: F, t1851: F, t7165: F, t11490: F, t11593: F, t11810: F, t11863: F, t1326: F, t137729: F, t137882: F, t138307: F, t144849: F, t1901: F, t1902: F, t23249: F, t23323: F, t23327: F, t23339: F, t25590: F, t26166: F, t26171: F, t26188: F, t26245: F, t26268: F, t26371: F, t26372: F, t26373: F, t26374: F, t26390: F, t26423: F, t3113: F, t3214: F, t3219: F, t32350: F, t34647: F, t34685: F, t34706: F, t379: F, t38651: F, t39150: F, t7274: F, t8372: F, t8557: F, t925: F) -> (F,) {
    let t145992 = t8392 * t34674;
    let t146020 = t8392 * t34700;
    let t146048 = t1851 * t7165;
    let t146061 = -2.0 / 27.0 * t145992 - 2.0 / 9.0 * t1901 * t11863 * t144849 - t1901 * t8557 * t32350 * t3113 / 9.0 + 2.0 / 9.0 * t1901 * t23323 * t26268 + 4.0 / 9.0 * t11593 * t23327 * t26188 + 2.0 / 27.0 * t137729 - 2.0 / 9.0 * t1901 * t39150 * t34685 + 8.0 * t1901 * t26372 * t38651 * t7274 * t3219 - 4.0 * t1901 * t26372 * t26373 * t25590 - 2.0 / 27.0 * t146020 - 2.0 / 9.0 * t1901 * t8557 * t34647 * t379 + t1901 * t8372 * t34706 / 9.0 + t1901 * t1902 * t137882 * t925 / 9.0 + 2.0 / 9.0 * t1901 * t23323 * t26245 - 4.0 * t1901 * t26371 * t1326 * t26374 - 4.0 / 3.0 * t1901 * t11490 * t26166 * t25590 + 2.0 * t1901 * t26171 * t138307 * t3214 + 4.0 / 3.0 * t1901 * t11810 * t146048 * t3219 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t26423 - 4.0 / 3.0 * t1901 * t11490 * t23249 * t26390;
    (t146061,)
}
