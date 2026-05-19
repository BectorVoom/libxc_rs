//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1214/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1214<F: Float>(t34162: F, t34179: F, t36962: F, t36967: F, t36968: F, t36969: F, t39049: F, t39052: F, t39054: F, t39057: F, t39060: F, t39062: F, t39064: F, t39069: F, t39071: F, t39073: F, t39075: F, t39077: F) -> F {
    let t41468 = F::cast_from(0.34299214494455789578e-2_f64) * t39049 - t36962 - F::cast_from(0.77173232612525526552e-1_f64) * t34162 - F::cast_from(0.34299214494455789578e-2_f64) * t39052 + F::cast_from(0.34299214494455789578e-2_f64) * t39054 + t39057 / F::new(64.0) + t39060 / F::new(64.0) - F::new(0.1120625e0) * t39062 - F::cast_from(0.37737710747524982483e-2_f64) * t39064 - F::cast_from(0.25158473831683321655e-2_f64) * t39069 - F::cast_from(0.68598428988911579156e-2_f64) * t39071 - F::cast_from(0.51448821741683684367e-2_f64) * t39073 + F::cast_from(0.2264262644851498949e-1_f64) * t39075 - t36967 + t36968 + F::cast_from(0.34299214494455789578e-2_f64) * t39077 + t36969 - F::cast_from(0.41930789719472202758e-2_f64) * t34179;
    t41468
}
