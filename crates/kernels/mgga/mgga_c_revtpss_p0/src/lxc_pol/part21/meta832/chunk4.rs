//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3109/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3109<F: Float>(t12956: F, t17217: F, t12909: F, t17395: F, t1042: F, t12277: F, t1261: F, t12777: F, t12781: F, t12822: F, t12828: F, t12836: F, t12842: F, t12847: F, t12912: F, t17235: F, t17237: F, t17448: F, t21049: F, t21306: F, t3647: F, t3711: F, t44343: F, t44346: F, t5277: F, t53450: F, t53474: F, t5381: F, t57114: F, t57118: F, t57126: F, t57128: F, t57136: F) -> F {
    let t57145 = t12956 * t17217;
    let t57147 = t12909 * t17395;
    let t57150 = -F::cast_from(0.42874018118069736972e-3_f64) * t17448 * t12836 - F::cast_from(0.42874018118069736972e-3_f64) * t17448 * t12777 - F::cast_from(0.85748036236139473944e-3_f64) * t21049 * t12842 + F::cast_from(0.42874018118069736972e-3_f64) * t21306 * t12847 - F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t12781 - F::cast_from(0.57165357490759649295e-3_f64) * t57114 + t44343 / F::new(216.0) + t44346 / F::new(108.0) + F::cast_from(0.95275595817932748826e-4_f64) * t57118 - F::cast_from(0.14291339372689912324e-3_f64) * t5381 * t12822 - F::cast_from(0.85748036236139473944e-3_f64) * t5381 * t12828 - t57126 + F::cast_from(0.57165357490759649295e-3_f64) * t57128 - F::cast_from(0.19055119163586549765e-2_f64) * t3647 * t17237 - F::cast_from(0.19055119163586549765e-2_f64) * t1261 * t1042 * t17235 * t53450 - F::cast_from(0.76220476654346199062e-2_f64) * t1261 * t1042 * t57136 * t53474 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t5277 * t12277 + F::cast_from(0.57165357490759649295e-3_f64) * t57145 - F::cast_from(0.68598428988911579154e-2_f64) * t57147 * t12912;
    t57150
}
