//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1054/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1054<F: Float>(t157: F, t1914: F, t2122: F, t2146: F, t2152: F, t26757: F, t29973: F, t29977: F, t29982: F, t32124: F, t33414: F, t33416: F, t33431: F, t33435: F, t33437: F, t38621: F, t38631: F, t38635: F, t38639: F, t7932: F, t8004: F, t8400: F, t9033: F) -> F {
    let t38641 = -F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8004 * t2122 * t1914 + t33414 - t33416 - t29973 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t38621 * t157 - F::cast_from(0.8673628188205199462e0_f64) * t8400 * t9033 * t26757 - F::cast_from(0.26020884564615598386e1_f64) * t29977 + t33431 + t33435 - t33437 - F::cast_from(0.17347256376410398924e1_f64) * t38631 - F::cast_from(0.69389025505641595696e1_f64) * t29982 + F::cast_from(0.26020884564615598386e1_f64) * t32124 * t7932 * t38635 + F::cast_from(0.17347256376410398924e1_f64) * t38639;
    t38641
}
