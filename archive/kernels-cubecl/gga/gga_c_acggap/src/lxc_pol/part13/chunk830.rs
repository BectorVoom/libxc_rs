//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 830/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk830<F: Float>(t1614: F, t1960: F, t119: F, t2331: F, t157: F, t2122: F, t524: F, t2152: F, t2333: F, t310: F, t1620: F, t2127: F, t2143: F, t2146: F, t2155: F, t2351: F, t464: F, t7900: F, t7901: F, t7909: F, t7921: F, t7926: F, t8995: F, t8999: F, t9003: F) -> (F, F, F) {
    let t9008 = t1960 * t1614;
    let t9010 = t119 * t2331;
    let t9014 = t2122 * t524 * t157;
    let t9015 = t2152 * t9014;
    let t9018 = t310 * t2333;
    let t9022 = F::cast_from(0.65854491829355115987e0_f64) * t119 * t8995 + F::cast_from(0.8673628188205199462e0_f64) * t8999 + t7900 + F::cast_from(0.65854491829355115987e0_f64) * t7901 - F::cast_from(0.17347256376410398924e1_f64) * t7909 + F::cast_from(0.4336814094102599731e0_f64) * t9003 * t2155 + F::cast_from(0.13170898365871023197e1_f64) * t2127 * t1620 + F::cast_from(0.65854491829355115987e0_f64) * t9008 - F::cast_from(0.65854491829355115987e0_f64) * t9010 * t464 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t9015 - t7921 + F::cast_from(0.65854491829355115987e0_f64) * t9018 - t7926 - F::cast_from(0.4336814094102599731e0_f64) * t2143 * t2351;
    (t9010, t9015, t9022)
}
