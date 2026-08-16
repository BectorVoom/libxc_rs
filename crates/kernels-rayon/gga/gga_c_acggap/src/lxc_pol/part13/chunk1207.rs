//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1207/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1207(t32041: f64, t36019: f64, t7932: f64, t2146: f64, t2147: f64, t2342: f64, t29994: f64, t32124: f64, t32222: f64, t32223: f64, t33597: f64, t33976: f64, t36531: f64, t36533: f64, t36541: f64, t36543: f64, t36547: f64, t463: f64, t7890: f64, t7912: f64, t8006: f64, t8393: f64, t8400: f64, t8437: f64, t8993: f64, t9003: f64, t944: f64) -> f64 {
    let t36555 = t32041 * t7932 * t36019;
    let t36566 = -0.26020884564615598386e1_f64 * t9003 * t8006 + 0.65854491829355115987e0_f64 * t36531 + 0.17347256376410398924e1_f64 * t36533 + 0.8673628188205199462e0_f64 * t29994 * t2342 + 0.17347256376410398924e1_f64 * t7912 * t8437 - 0.8673628188205199462e0_f64 * t36541 + 0.8673628188205199462e0_f64 * t36543 + 0.17347256376410398924e1_f64 * t7912 * t8393 + 0.26020884564615598386e1_f64 * t32124 * t7932 * t36547 + 0.4336814094102599731e0_f64 * t8400 * t7932 * t33976 + 0.26020884564615598386e1_f64 * t36555 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t8993 * t463 - 0.8673628188205199462e0_f64 * t2146 * t7890 * t33597 * t944 + t32222 - 0.34694512752820797848e1_f64 * t32223;
    t36566
}
