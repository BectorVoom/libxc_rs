//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1477/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477<F: Float>(t31555: F, t508: F, t569: F, t1911: F, t8362: F, t1843: F, t1312: F, t18245: F, t2179: F, t2181: F, t29508: F, t30138: F, t30143: F, t31518: F, t31533: F, t4248: F, t651: F, t7732: F, t7889: F, t8353: F, t8363: F, t8367: F, t8369: F) -> (F, F, F, F, F) {
    let t31556 = t508 * t31555;
    let t31567 = t31555 * t569;
    let t31570 = t8362 * t1911;
    let t31579 = t1843 * t8362;
    let t31582 = F::new(2.0) * t1312 * t31533 + F::new(2.0) * t1312 * t31567 + F::new(4.0) * t1312 * t31570 - F::new(2.0) * t18245 * t2179 + F::new(2.0) * t18245 * t2181 - F::new(2.0) * t2179 * t29508 - F::new(4.0) * t2179 * t30138 + F::new(4.0) * t2181 * t30138 + F::new(2.0) * t2181 * t30143 - F::new(2.0) * t31518 * t651 - F::new(2.0) * t31556 * t651 - F::new(4.0) * t31579 * t651 - F::new(4.0) * t4248 * t8353 - F::new(4.0) * t4248 * t8363 + F::new(4.0) * t4248 * t8367 + F::new(4.0) * t4248 * t8369 - F::new(4.0) * t7732 * t8353 - F::new(4.0) * t7732 * t8363 + F::new(4.0) * t7889 * t8367 + F::new(4.0) * t7889 * t8369;
    (t31556, t31567, t31570, t31579, t31582)
}
