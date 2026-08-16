//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1208/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1208<F: Float>(t33489: F, t7963: F, t9029: F, t2137: F, t40619: F, t2140: F, t2131: F, t2147: F, t309: F, t9789: F, t157: F, t1838: F, t1938: F, t2122: F, t2127: F, t2146: F, t2152: F, t2159: F, t31965: F, t31969: F, t31976: F, t33606: F, t33621: F, t33624: F, t33627: F, t524: F, t6425: F, t8001: F, t8993: F, t9509: F, t9517: F) -> F {
    let t40645 = t7963 * t33489 * t9029;
    let t40653 = t2137 * t40619;
    let t40654 = t40653 * t2140;
    let t40664 = t2131 * t2147 * t9789 * t309;
    let t40666 = F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t2122 * t1838 * t157 + F::cast_from(0.13170898365871023197e1_f64) * t31969 - F::cast_from(0.4336814094102599731e0_f64) * t9517 * t2159 - t33606 + F::cast_from(0.17347256376410398924e1_f64) * t40645 + F::cast_from(0.26341796731742046394e1_f64) * t2127 * t6425 + t33621 - t33624 - F::cast_from(0.65854491829355115987e0_f64) * t8001 * t1938 - F::cast_from(0.17347256376410398924e1_f64) * t31965 * t9509 + F::cast_from(0.8673628188205199462e0_f64) * t40654 - t33627 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2152 * t8993 * t524 * t157 + F::cast_from(0.34694512752820797848e1_f64) * t31976 + F::cast_from(0.34694512752820797848e1_f64) * t40664;
    t40666
}
