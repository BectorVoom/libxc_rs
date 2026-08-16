//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1208/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1208(t33489: f64, t7963: f64, t9029: f64, t2137: f64, t40619: f64, t2140: f64, t2131: f64, t2147: f64, t309: f64, t9789: f64, t157: f64, t1838: f64, t1938: f64, t2122: f64, t2127: f64, t2146: f64, t2152: f64, t2159: f64, t31965: f64, t31969: f64, t31976: f64, t33606: f64, t33621: f64, t33624: f64, t33627: f64, t524: f64, t6425: f64, t8001: f64, t8993: f64, t9509: f64, t9517: f64) -> f64 {
    let t40645 = t7963 * t33489 * t9029;
    let t40653 = t2137 * t40619;
    let t40654 = t40653 * t2140;
    let t40664 = t2131 * t2147 * t9789 * t309;
    let t40666 = 0.4336814094102599731e0_f64 * t2146 * t2152 * t2122 * t1838 * t157 + 0.13170898365871023197e1_f64 * t31969 - 0.4336814094102599731e0_f64 * t9517 * t2159 - t33606 + 0.17347256376410398924e1_f64 * t40645 + 0.26341796731742046394e1_f64 * t2127 * t6425 + t33621 - t33624 - 0.65854491829355115987e0_f64 * t8001 * t1938 - 0.17347256376410398924e1_f64 * t31965 * t9509 + 0.8673628188205199462e0_f64 * t40654 - t33627 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t8993 * t524 * t157 + 0.34694512752820797848e1_f64 * t31976 + 0.34694512752820797848e1_f64 * t40664;
    t40666
}
