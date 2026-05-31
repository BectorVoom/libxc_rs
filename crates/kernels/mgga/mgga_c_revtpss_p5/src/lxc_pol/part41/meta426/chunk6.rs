//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1492/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1492<F: Float>(t108710: F, t108714: F, t109150: F, t109153: F, t118407: F, t1312: F, t13426: F, t18227: F, t1911: F, t2179: F, t2181: F, t2322: F, t29508: F, t30138: F, t30143: F, t31292: F, t31309: F, t31320: F, t31533: F, t31567: F, t31570: F, t4248: F, t5523: F, t569: F, t6934: F, t8254: F, t8273: F, t8274: F, t8278: F, t8280: F, t8369: F) -> F {
    let t118500 = F::cast_from(2.0_f64) * t118407 * t1312 * t569 + F::cast_from(4.0_f64) * t1312 * t1911 * t31292 + F::cast_from(2.0_f64) * t1312 * t6934 * t8273 - F::cast_from(2.0_f64) * t108710 * t2179 - F::cast_from(2.0_f64) * t108714 * t2179 + F::cast_from(4.0_f64) * t109150 * t2181 + F::cast_from(4.0_f64) * t109153 * t2181 + F::cast_from(4.0_f64) * t13426 * t8369 + F::cast_from(4.0_f64) * t18227 * t8369 + F::cast_from(2.0_f64) * t2322 * t31533 + F::cast_from(2.0_f64) * t2322 * t31567 - F::cast_from(2.0_f64) * t29508 * t8254 - F::cast_from(2.0_f64) * t29508 * t8274 + F::cast_from(4.0_f64) * t30138 * t8278 + F::cast_from(2.0_f64) * t30143 * t8280 + F::cast_from(4.0_f64) * t31309 * t4248 - F::cast_from(4.0_f64) * t31320 * t4248 + F::cast_from(2.0_f64) * t31533 * t5523 + F::cast_from(2.0_f64) * t31567 * t5523 + F::cast_from(4.0_f64) * t31570 * t5523;
    t118500
}
