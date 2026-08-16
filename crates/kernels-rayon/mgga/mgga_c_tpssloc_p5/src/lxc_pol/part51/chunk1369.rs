//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1369/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1369(t15868: f64, t1983: f64, t8640: f64, t121019: f64, t121129: f64, t121132: f64, t121134: f64, t121136: f64, t121138: f64, t121142: f64, t23938: f64, t26898: f64, t26902: f64, t32674: f64, t32676: f64, t32679: f64, t510: f64, t7472: f64, t8450: f64) -> f64 {
    let t121144 = t1983 * t8640 * t15868;
    let t121149 = -t121129 * t510 - 2.0_f64 * t23938 * t7472 + 3.0_f64 * t26898 * t8450 - t26902 * t8450 - t121019 + t121132 - t121134 - t121136 - t121138 + t121142 - t121144 - t32674 - t32676 - t32679;
    t121149
}
