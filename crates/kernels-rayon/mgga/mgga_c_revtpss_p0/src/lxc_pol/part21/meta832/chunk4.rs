//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3109/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3109(t12956: f64, t17217: f64, t12909: f64, t17395: f64, t1042: f64, t12277: f64, t1261: f64, t12777: f64, t12781: f64, t12822: f64, t12828: f64, t12836: f64, t12842: f64, t12847: f64, t12912: f64, t17235: f64, t17237: f64, t17448: f64, t21049: f64, t21306: f64, t3647: f64, t3711: f64, t44343: f64, t44346: f64, t5277: f64, t53450: f64, t53474: f64, t5381: f64, t57114: f64, t57118: f64, t57126: f64, t57128: f64, t57136: f64) -> f64 {
    let t57145 = t12956 * t17217;
    let t57147 = t12909 * t17395;
    let t57150 = -0.42874018118069736972e-3_f64 * t17448 * t12836 - 0.42874018118069736972e-3_f64 * t17448 * t12777 - 0.85748036236139473944e-3_f64 * t21049 * t12842 + 0.42874018118069736972e-3_f64 * t21306 * t12847 - 0.85748036236139473944e-3_f64 * t17448 * t12781 - 0.57165357490759649295e-3_f64 * t57114 + t44343 / 216.0_f64 + t44346 / 108.0_f64 + 0.95275595817932748826e-4_f64 * t57118 - 0.14291339372689912324e-3_f64 * t5381 * t12822 - 0.85748036236139473944e-3_f64 * t5381 * t12828 - t57126 + 0.57165357490759649295e-3_f64 * t57128 - 0.19055119163586549765e-2_f64 * t3647 * t17237 - 0.19055119163586549765e-2_f64 * t1261 * t1042 * t17235 * t53450 - 0.76220476654346199062e-2_f64 * t1261 * t1042 * t57136 * t53474 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t5277 * t12277 + 0.57165357490759649295e-3_f64 * t57145 - 0.68598428988911579154e-2_f64 * t57147 * t12912;
    t57150
}
