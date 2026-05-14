//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1075/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1075<F: Float>(t32003: F, t36479: F, t38086: F, t2385: F, t394: F, t2132: F, t7885: F, t864: F, t2146: F, t2147: F, t31965: F, t33034: F, t33037: F, t33047: F, t33053: F, t38073: F, t38077: F, t38085: F, t463: F, t7912: F, t7931: F, t7934: F, t8069: F, t8119: F, t9003: F, t9150: F, t9162: F, t9367: F) -> (F, F) {
    let t38089 = 0.34694512752820797848e1 * t32003 * t38086 * t36479;
    let t38092 = t394 * t2385;
    let t38104 = t7885 * t2132 * t2385 * t864;
    let t38108 = 0.8673628188205199462e0 * t38073 - 0.8673628188205199462e0 * t9003 * t8119 - 0.65854491829355115987e0 * t38077 + 0.17347256376410398924e1 * t33034 - 0.52041769129231196772e1 * t7912 * t9150 + t33037 - t38085 + t38089 - 0.17347256376410398924e1 * t31965 * t9162 - 0.17347256376410398924e1 * t7931 * t38092 * t7934 + 0.17347256376410398924e1 * t2146 * t2147 * t9367 * t463 + 0.17347256376410398924e1 * t9003 * t8069 - 0.26020884564615598386e1 * t38104 + 0.26341796731742046394e1 * t33047 + 0.34694512752820797848e1 * t33053;
    (t38092, t38108)
}
