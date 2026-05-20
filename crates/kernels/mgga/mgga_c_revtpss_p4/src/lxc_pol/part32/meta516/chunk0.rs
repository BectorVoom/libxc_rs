//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1818/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1818<F: Float>(t4147: F, t7535: F, t36: F, t68: F, t1518: F, t2051: F, t2055: F, t8107: F, t1469: F, t1450: F, t211: F, t9644: F) -> (F, F, F, F, F, F, F) {
    let t33183 = t4147 * t7535;
    let t33268 = t68 * t36;
    let t34251 = t2051 * t1518;
    let t34359 = t1518 * t2055;
    let t34495 = t4147 * t8107;
    let t34764 = t33268 * t1469;
    let t35927 = t8107 * t1450;
    let t39643 = F::new(1.0) / t9644 / t211;
    (t33183, t34251, t34359, t34495, t34764, t35927, t39643)
}
