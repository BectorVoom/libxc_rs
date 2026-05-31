//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1305/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1305<F: Float>(t3382: F, t5807: F, t1077: F, t1180: F, t1181: F, t127: F, t129: F, t14405: F, t14414: F, t14419: F, t14421: F, t14429: F, t145: F, t1552: F, t1759: F, t18817: F, t18819: F, t18828: F, t18830: F, t22607: F, t5: F) -> F {
    let t24294 = t3382 * t5807;
    let t24296 = F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t1552 * t1759 * t1077 - F::cast_from(0.85748036236139473944e-3_f64) * t18817 - F::cast_from(0.32012600194825403606e-1_f64) * t18819 - t14405 - t14414 - t14419 - t14421 + F::cast_from(0.1133779590233399711e0_f64) * t14429 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t18828 + t127 * t129 * t5 * t22607 * t145 / F::cast_from(96.0_f64) - F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t18830 + F::cast_from(0.17149607247227894789e-2_f64) * t24294;
    t24296
}
