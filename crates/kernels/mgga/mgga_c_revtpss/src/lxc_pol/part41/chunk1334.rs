//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1334/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1334<F: Float>(t31292: F, t569: F, t2178: F, t5517: F, t1312: F, t13426: F, t18227: F, t2179: F, t2181: F, t2322: F, t27123: F, t28219: F, t31248: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t7889: F, t8274: F, t8278: F, t8280: F, t8353: F, t8367: F) -> (F, F, F) {
    let t31293 = t31292 * t569;
    let t31299 = t5517 * t2178;
    let t31303 = t1312 * t31248 + t1312 * t31293 - t13426 * t2179 - t18227 * t2179 + t2181 * t27123 + t2181 * t28219 - t2322 * t8353 + t2322 * t8367 - t31299 * t651 - t4248 * t8274 + t4248 * t8280 - t4254 * t8353 + t5523 * t8367 - t7732 * t8274 + t7889 * t8278 + t7889 * t8280;
    (t31293, t31299, t31303)
}
