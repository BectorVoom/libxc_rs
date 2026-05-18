//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 990/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk990<F: Float>(t1264: F, t1410: F, t157: F, t2122: F, t2127: F, t2146: F, t2152: F, t2341: F, t31944: F, t31951: F, t31955: F, t31969: F, t31972: F, t33597: F, t33606: F, t33621: F, t33624: F, t33627: F, t406: F, t5332: F, t7912: F, t8004: F, t8433: F, t8993: F) -> F {
    let t33628 = F::new(0.52041769129231196772e1) * t31944 + F::new(0.34694512752820797848e1) * t31951 + F::new(0.8673628188205199462e0) * t2146 * t2152 * t8993 * t406 * t157 - F::new(0.52041769129231196772e1) * t31955 + F::new(0.4336814094102599731e0) * t2146 * t2152 * t33597 * t157 + F::new(0.26341796731742046394e1) * t31969 + F::new(0.8673628188205199462e0) * t7912 * t8433 - t33606 + F::new(0.17347256376410398924e1) * t31972 - F::new(0.65854491829355115987e0) * t2127 * t5332 - F::new(0.26020884564615598386e1) * t2146 * t8004 * t2341 * t1264 + F::new(0.8673628188205199462e0) * t2146 * t2152 * t2122 * t1410 * t157 + t33621 - t33624 - t33627;
    t33628
}
