//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 971/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk971<F: Float>(t508: F, t8362: F, t569: F, t1911: F, t2178: F, t1312: F, t2179: F, t2181: F, t4248: F, t651: F, t7732: F, t7889: F, t8353: F) -> (F, F, F, F) {
    let t8363 = t508 * t8362;
    let t8367 = t8362 * t569;
    let t8369 = t2178 * t1911;
    let t8372 = F::cast_from(2.0_f64) * t1312 * t8367 + F::cast_from(2.0_f64) * t1312 * t8369 - F::cast_from(2.0_f64) * t2179 * t4248 - F::cast_from(2.0_f64) * t2179 * t7732 + F::cast_from(2.0_f64) * t2181 * t4248 + F::cast_from(2.0_f64) * t2181 * t7889 - F::cast_from(2.0_f64) * t651 * t8353 - F::cast_from(2.0_f64) * t651 * t8363;
    (t8363, t8367, t8369, t8372)
}
