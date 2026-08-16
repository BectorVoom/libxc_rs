//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1214/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1214(t34162: f64, t34179: f64, t36962: f64, t36967: f64, t36968: f64, t36969: f64, t39049: f64, t39052: f64, t39054: f64, t39057: f64, t39060: f64, t39062: f64, t39064: f64, t39069: f64, t39071: f64, t39073: f64, t39075: f64, t39077: f64) -> f64 {
    let t41468 = 0.34299214494455789578e-2_f64 * t39049 - t36962 - 0.77173232612525526552e-1_f64 * t34162 - 0.34299214494455789578e-2_f64 * t39052 + 0.34299214494455789578e-2_f64 * t39054 + t39057 / 64.0_f64 + t39060 / 64.0_f64 - 0.1120625e0_f64 * t39062 - 0.37737710747524982483e-2_f64 * t39064 - 0.25158473831683321655e-2_f64 * t39069 - 0.68598428988911579156e-2_f64 * t39071 - 0.51448821741683684367e-2_f64 * t39073 + 0.2264262644851498949e-1_f64 * t39075 - t36967 + t36968 + 0.34299214494455789578e-2_f64 * t39077 + t36969 - 0.41930789719472202758e-2_f64 * t34179;
    t41468
}
