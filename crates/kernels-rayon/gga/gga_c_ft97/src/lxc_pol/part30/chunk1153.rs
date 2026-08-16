//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1153/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1153(t7114: f64, t98724: f64, t1882: f64, t36228: f64, t112883: f64, t114820: f64, t143653: f64, t1476: f64, t152861: f64, t15294: f64, t153372: f64, t15369: f64, t15460: f64, t1901: f64, t2347: f64, t24898: f64, t2749: f64, t2766: f64, t2862: f64, t28924: f64, t29137: f64, t29369: f64, t296: f64, t319: f64, t34159: f64, t35972: f64, t36003: f64, t36121: f64, t36152: f64, t3886: f64, t4141: f64, t4261: f64, t4266: f64, t446: f64, t56098: f64, t56815: f64, t6386: f64, t6393: f64, t7021: f64, t7036: f64, t7662: f64, t7679: f64, t824: f64, t840: f64, t871: f64, t875: f64, t882: f64) -> (f64, f64) {
    let t154017 = t98724 * t7114;
    let t154059 = t1882 * t36228;
    let t154082 = 2.0_f64 / 27.0_f64 * t1901 * t15294 * t7679 * t2347 * t3886 - 2.0_f64 / 9.0_f64 * t1901 * t56098 * t34159 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t6393 * t7036 + 4.0_f64 / 3.0_f64 * t446 * t296 * t154017 + 2.0_f64 / 3.0_f64 * t446 * t840 * t871 * t7021 * t6386 - t446 * t840 * t36003 * t824 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t840 * t871 * t1476 * t28924 - 4.0_f64 / 3.0_f64 * t1901 * t15369 * t24898 * t29369 - 4.0_f64 / 3.0_f64 * t1901 * t56815 * t36152 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t112883 * t7114 + t1901 * t143653 * t4261 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t143653 * t4266 - 2.0_f64 / 27.0_f64 * t1901 * t2766 * t7662 * t4141 - 4.0_f64 / 3.0_f64 * t1901 * t114820 * t29137 + t154059 / 9.0_f64 - t446 * t840 * t882 * t35972 / 3.0_f64 - t446 * t840 * t319 * t153372 / 3.0_f64 + t446 * t840 * t871 * t35972 * t875 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t319 * t152861 + t446 * t840 * t2749 * t36121 / 3.0_f64;
    (t154017, t154082)
}
