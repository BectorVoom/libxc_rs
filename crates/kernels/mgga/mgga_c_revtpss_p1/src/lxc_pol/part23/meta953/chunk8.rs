//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3171/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3171<F: Float>(t1038: F, t1241: F, t1244: F, t24679: F, t1252: F, t17351: F, t17649: F, t17693: F, t17799: F, t1797: F, t21028: F, t21102: F, t5287: F, t57118: F, t69958: F, t70082: F, t70088: F, t70369: F, t70373: F, t70376: F, t83033: F, t83034: F) -> F {
    let t83296 = t1241 * t1244 * t24679 * t1038;
    let t83307 = -F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t17799 * t83034 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t17649 * t83033 * t21028 + F::cast_from(0.21722835846488666732e-1_f64) * t70082 * t1797 + F::cast_from(0.21722835846488666732e-1_f64) * t21102 * t5287 - F::cast_from(0.53100265402527852012e-1_f64) * t83296 * t1252 + F::cast_from(0.64311027177104605458e-3_f64) * t69958 * t1797 - F::cast_from(0.68598428988911579154e-2_f64) * t70088 * t1797 + F::cast_from(0.95275595817932748825e-4_f64) * t57118 - F::cast_from(0.11433071498151929859e-2_f64) * t70369 + F::cast_from(0.85748036236139473944e-3_f64) * t70373 - F::cast_from(0.17149607247227894788e-2_f64) * t70376;
    t83307
}
