//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2213/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2213<F: Float>(t22061: F, t25986: F, t2661: F, t22026: F, t94550: F, t22052: F, t7271: F, t22056: F, t25972: F, t94520: F, t94523: F, t94526: F, t94527: F, t94537: F, t94540: F, t94546: F, t98270: F) -> F {
    let t108601 = t2661 * t25986 * t22061;
    let t108604 = t2661 * t94550 * t22026;
    let t108606 = t7271 * t22052;
    let t108608 = t25972 * t22056;
    let t108613 = -F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t94520 - t94523 + t94526 - F::cast_from(0.60976381323476959248e-3_f64) * t94527 + F::cast_from(0.14291339372689912324e-4_f64) * t108601 - F::cast_from(0.28582678745379824648e-4_f64) * t108604 - F::cast_from(0.17149607247227894789e-2_f64) * t108606 - F::cast_from(0.10164000561857065645e-3_f64) * t108608 + F::cast_from(0.50820002809285328225e-5_f64) * t94537 - F::cast_from(0.36143185997963725434e-4_f64) * t94540 + t98270 - F::cast_from(0.45351183609335988444e-1_f64) * t94546;
    t108613
}
