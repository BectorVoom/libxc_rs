//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 647/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk647<F: Float>(t1081: F, t2975: F, t2834: F, t2891: F, t2836: F, t2843: F, t2848: F, t2852: F, t2870: F, t2878: F, t2886: F, t2888: F, t2893: F, t2897: F, t2900: F, t2903: F) -> (F, F, F, F) {
    let t2976 = t2975 * t1081;
    let t2981 = F::cast_from(0.40256666666666666667e0_f64) * t2834;
    let t2988 = F::cast_from(0.137975e0_f64) * t2891;
    let t2993 = -F::cast_from(0.1294625e1_f64) * t2870 + F::cast_from(0.258925e1_f64) * t2878 + t2981 - F::cast_from(0.20128333333333333334e0_f64) * t2836 - F::cast_from(0.20128333333333333333e0_f64) * t2843 + F::cast_from(0.60385e0_f64) * t2848 + F::cast_from(0.301925e0_f64) * t2852 + F::cast_from(0.82524375e-1_f64) * t2886 + F::cast_from(0.16504875e0_f64) * t2888 + t2988 - F::cast_from(0.11038e0_f64) * t2893 - F::cast_from(0.27595e-1_f64) * t2897 + F::cast_from(0.16557e0_f64) * t2900 + F::cast_from(0.82785e-1_f64) * t2903;
    (t2976, t2981, t2988, t2993)
}
