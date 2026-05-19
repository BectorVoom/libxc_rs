//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1204/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1204<F: Float>(t119982: F, t127676: F, t31837: F, t32471: F, t98848: F, t126110: F, t119877: F, t119879: F, t121879: F, t121881: F, t121886: F, t126182: F, t126185: F, t1949: F, t28340: F, t8649: F, t8650: F) -> F {
    let t127684 = t119982 * t127676;
    let t127689 = t98848 * t31837 * t32471;
    let t127692 = t126110 * t31837 * t32471;
    let t127694 = t121879 - t121881 + F::cast_from(0.57119737665102352616e0_f64) * t8649 * t8650 * t28340 * t1949 + F::cast_from(0.42839803248826764462e-1_f64) * t127684 + t119877 + t119879 - t121886 + F::cast_from(0.225875734067843736e-2_f64) * t126182 - F::cast_from(0.69416347856895220197e-2_f64) * t126185 - F::cast_from(0.14279934416275588154e-1_f64) * t127689 + F::cast_from(0.25389723392137995738e-1_f64) * t127692;
    t127694
}
