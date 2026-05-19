//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 738/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk738<F: Float>(t5: F, t1923: F, t2123: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F, t117: F, t116: F, t2126: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7583 = piecewise3::<F>(t8, F::new(0.0), -t6954 * t2123 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t7566 * t6960 + t6963 * t2123 / F::new(3.0) - t1923 * t7576 / F::new(6.0) - t1923 * t7579 / F::new(6.0));
    let t7584 = t7583 * t117;
    let t7586 = t2126 * t116;
    (t7583, t7584, t7586)
}
