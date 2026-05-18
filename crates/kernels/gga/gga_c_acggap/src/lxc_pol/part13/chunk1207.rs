//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1207/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1207<F: Float>(t32041: F, t36019: F, t7932: F, t2146: F, t2147: F, t2342: F, t29994: F, t32124: F, t32222: F, t32223: F, t33597: F, t33976: F, t36531: F, t36533: F, t36541: F, t36543: F, t36547: F, t463: F, t7890: F, t7912: F, t8006: F, t8393: F, t8400: F, t8437: F, t8993: F, t9003: F, t944: F) -> F {
    let t36555 = t32041 * t7932 * t36019;
    let t36566 = -F::new(0.26020884564615598386e1) * t9003 * t8006 + F::new(0.65854491829355115987e0) * t36531 + F::new(0.17347256376410398924e1) * t36533 + F::new(0.8673628188205199462e0) * t29994 * t2342 + F::new(0.17347256376410398924e1) * t7912 * t8437 - F::new(0.8673628188205199462e0) * t36541 + F::new(0.8673628188205199462e0) * t36543 + F::new(0.17347256376410398924e1) * t7912 * t8393 + F::new(0.26020884564615598386e1) * t32124 * t7932 * t36547 + F::new(0.4336814094102599731e0) * t8400 * t7932 * t33976 + F::new(0.26020884564615598386e1) * t36555 + F::new(0.17347256376410398924e1) * t2146 * t2147 * t8993 * t463 - F::new(0.8673628188205199462e0) * t2146 * t7890 * t33597 * t944 + t32222 - F::new(0.34694512752820797848e1) * t32223;
    t36566
}
