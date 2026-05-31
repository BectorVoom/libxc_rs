//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 717/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk717<F: Float>(t2988: F, t973: F, t2846: F, t2904: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F) -> (F, F, F, F) {
    let t2989 = t2988 * t973;
    let t2994 = F::cast_from(0.40256666666666666667e0_f64) * t2846;
    let t3001 = F::cast_from(0.137975e0_f64) * t2904;
    let t3006 = -F::cast_from(0.1294625e1_f64) * t2882 + F::cast_from(0.258925e1_f64) * t2890 + t2994 + F::cast_from(0.20128333333333333334e0_f64) * t2848 - F::cast_from(0.20128333333333333333e0_f64) * t2855 + F::cast_from(0.60385e0_f64) * t2860 - F::cast_from(0.301925e0_f64) * t2864 + F::cast_from(0.82524375e-1_f64) * t2898 + F::cast_from(0.16504875e0_f64) * t2900 + t3001 + F::cast_from(0.11038e0_f64) * t2906 - F::cast_from(0.27595e-1_f64) * t2910 + F::cast_from(0.16557e0_f64) * t2913 - F::cast_from(0.82785e-1_f64) * t2916;
    (t2989, t2994, t3001, t3006)
}
