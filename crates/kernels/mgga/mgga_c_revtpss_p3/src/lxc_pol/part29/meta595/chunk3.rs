//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1998/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1998<F: Float>(t136: F, t2457: F, t8006: F, t93377: F, t28314: F, t93342: F, t28417: F, t686: F, t72: F, t25375: F, t2435: F, t8011: F) -> (F, F, F, F, F, F) {
    let t102980 = t8006 * t136 * t2457;
    let t102981 = t93377 * t102980;
    let t102984 = F::cast_from(0.51405703062096148812e-1_f64) * t93342 * t28314;
    let t102986 = t28417 * t72 * t686;
    let t102988 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t102986;
    let t102993 = t8011 * t2435;
    (t102980, t102981, t102984, t102986, t102988, t102993)
}
