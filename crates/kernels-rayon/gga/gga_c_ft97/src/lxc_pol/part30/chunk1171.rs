//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1171/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1171(t1882: f64, t36138: f64, t36224: f64, t36204: f64, t114578: f64, t126613: f64, t144005: f64, t144131: f64, t144162: f64, t144176: f64, t144178: f64, t144184: f64, t144190: f64, t15369: f64, t153723: f64, t15460: f64, t1901: f64, t29185: f64, t29302: f64, t29307: f64, t296: f64, t3281: f64, t34102: f64, t36042: f64, t3746: f64, t4176: f64, t4246: f64, t4256: f64, t446: f64, t53797: f64, t6260: f64, t6353: f64, t6365: f64, t6374: f64, t7131: f64, t7686: f64, t824: f64, t835: f64, t840: f64, t871: f64, t99672: f64) -> f64 {
    let t154728 = t1882 * t36138;
    let t154734 = t1882 * t36224;
    let t154738 = t1882 * t36204;
    let t154783 = t144162 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t154728 - 2.0_f64 / 3.0_f64 * t446 * t840 * t7131 * t6260 + t154734 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t144176 + 2.0_f64 / 9.0_f64 * t144178 + 2.0_f64 / 9.0_f64 * t154738 + 2.0_f64 / 3.0_f64 * t446 * t296 * t153723 + 2.0_f64 / 9.0_f64 * t3281 * t835 * t7686 * t3746 + 2.0_f64 / 3.0_f64 * t446 * t840 * t4246 * t34102 + 4.0_f64 / 9.0_f64 * t53797 * t99672 * t29185 - 4.0_f64 / 3.0_f64 * t1901 * t15369 * t114578 * t6365 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t126613 * t6374 + t1901 * t144005 * t4256 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t15460 * t144131 * t4176 + t144184 + t446 * t840 * t871 * t36042 * t824 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t840 * t6353 * t29302 + 2.0_f64 / 3.0_f64 * t446 * t840 * t6353 * t29307 - t144190 / 9.0_f64;
    t154783
}
