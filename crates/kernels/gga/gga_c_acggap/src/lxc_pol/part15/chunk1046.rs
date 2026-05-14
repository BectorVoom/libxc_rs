//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1046/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1046<F: Float>(t39499: F, t7942: F, t8306: F, t38226: F, t557: F, t310: F, t9973: F, t40703: F, t7963: F, t1938: F, t8331: F, t2146: F, t30023: F, t33150: F, t33153: F, t33157: F, t33778: F, t38215: F, t38224: F, t38662: F, t40749: F, t463: F, t7931: F, t9003: F, t9145: F, t9162: F, t9402: F, t9976: F) -> (F,) {
    let t41196 = t7942 * t8306 * t39499;
    let t41200 = t38226 * t557;
    let t41211 = t310 * t9973;
    let t41214 = t7963 * t8306 * t40703;
    let t41216 = t8331 * t1938;
    let t41225 = -0.8673628188205199462e0 * t41196 - 0.17347256376410398924e1 * t33778 * t9162 - 0.13170898365871023197e1 * t41200 + 0.8673628188205199462e0 * t9003 * t9145 + 0.52041769129231196772e1 * t38215 - 0.17347256376410398924e1 * t7931 * t8306 * t40749 - 0.69389025505641595696e1 * t33150 + 0.26020884564615598386e1 * t33153 + 0.34694512752820797848e1 * t33157 + 0.65854491829355115987e0 * t41211 + 0.8673628188205199462e0 * t41214 - 0.65854491829355115987e0 * t41216 + 0.10408353825846239354e2 * t2146 * t30023 * t9976 * t463 + 0.8673628188205199462e0 * t38662 * t9402 - 0.69389025505641595696e1 * t38224;
    (t41225,)
}
