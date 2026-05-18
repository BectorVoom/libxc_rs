//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 848/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk848<F: Float>(t157: F, t2385: F, t524: F, t2152: F, t159: F, t619: F, t9971: F, t119: F, t2146: F, t2338: F, t2400: F, t2404: F, t616: F, t8067: F, t8087: F, t8096: F, t8098: F, t8106: F, t8113: F, t9003: F, t9129: F, t9155: F, t9160: F, t9169: F, t9172: F, t9973: F, t9977: F, t9982: F, t9986: F) -> (F, F, F) {
    let t9990 = t2385 * t524 * t157;
    let t9991 = t2152 * t9990;
    let t9995 = t619 * t159 * t9971;
    let t10004 = t8067 - F::new(0.8673628188205199462e0) * t2338 * t2404 + F::new(0.13170898365871023197e1) * t9129 + F::new(0.65854491829355115987e0) * t119 * t9973 + t8087 - F::new(0.26020884564615598386e1) * t2146 * t9977 - F::new(0.8673628188205199462e0) * t2146 * t9982 + F::new(0.17347256376410398924e1) * t2146 * t9986 + F::new(0.8673628188205199462e0) * t2146 * t9991 - F::new(0.4336814094102599731e0) * t616 * t9995 - t8096 + F::new(0.8673628188205199462e0) * t9003 * t2400 - F::new(0.34694512752820797848e1) * t9155 - t8098 - t8106 + F::new(0.34694512752820797848e1) * t9160 + F::new(0.17347256376410398924e1) * t9169 - F::new(0.17347256376410398924e1) * t9172 - t8113;
    (t9991, t9995, t10004)
}
