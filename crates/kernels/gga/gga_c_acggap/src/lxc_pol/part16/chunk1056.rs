//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1056/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1056<F: Float>(t1839: F, t463: F, t119: F, t9767: F, t2146: F, t2147: F, t30036: F, t33475: F, t33488: F, t33489: F, t33496: F, t33500: F, t33504: F, t33507: F, t38621: F, t464: F, t556: F, t7890: F, t7931: F, t7932: F, t8004: F, t8400: F, t8402: F, t8993: F, t9025: F, t944: F, t9793: F) -> F {
    let t38685 = t1839 * t463;
    let t38689 = t119 * t9767;
    let t38693 = -F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8004 * t9793 * t463 - t33475 - F::cast_from(0.8673628188205199462e0_f64) * t2146 * t7890 * t38621 * t944 + F::cast_from(0.17347256376410398924e1_f64) * t2146 * t2147 * t8993 * t556 - t30036 + t33488 + t33496 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t33489 * t9025 + F::cast_from(0.8673628188205199462e0_f64) * t8400 * t33489 * t8402 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t7932 * t38685 - F::cast_from(0.65854491829355115987e0_f64) * t38689 * t464 - t33500 + t33504 + F::cast_from(0.10408353825846239354e2_f64) * t33507;
    t38693
}
