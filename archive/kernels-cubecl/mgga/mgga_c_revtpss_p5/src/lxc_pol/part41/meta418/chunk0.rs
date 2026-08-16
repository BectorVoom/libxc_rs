//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1472/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1472<F: Float>(t1911: F, t8273: F, t1843: F, t1310: F, t8362: F, t31292: F, t508: F, t2178: F, t5787: F, t1312: F, t13426: F, t18227: F, t2179: F, t2181: F, t2322: F, t27123: F, t27126: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t8254: F, t8278: F, t8363: F, t8369: F) -> (F, F, F, F, F, F) {
    let t31309 = t8273 * t1911;
    let t31314 = t1843 * t8273;
    let t31318 = t1310 * t8362;
    let t31320 = t508 * t31292;
    let t31324 = t2178 * t5787;
    let t31326 = t1312 * t31309 + t1312 * t31324 + t13426 * t2181 + t18227 * t2181 - t2179 * t27123 - t2179 * t27126 - t2322 * t8363 + t2322 * t8369 - t31314 * t651 - t31318 * t651 - t31320 * t651 - t4248 * t8254 + t4248 * t8278 - t4254 * t8363 + t5523 * t8369 - t7732 * t8254;
    (t31309, t31314, t31318, t31320, t31324, t31326)
}
