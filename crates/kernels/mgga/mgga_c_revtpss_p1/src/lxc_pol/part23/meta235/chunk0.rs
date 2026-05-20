//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1379/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1379<F: Float>(t30: F, t33: F, t512: F, t6801: F, t1344: F, t3874: F, t5824: F, t6785: F, t1348: F, t3881: F, t6416: F, t6792: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t6802 = t512 * t6801;
    let t6808 = piecewise3::<F>(t31, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3874 * t6785 + F::new(2.0) / F::new(3.0) * t1344 * t5824);
    let t6814 = piecewise3::<F>(t34, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3881 * t6792 + F::new(2.0) / F::new(3.0) * t1348 * t6416);
    let t6816 = t6808 / F::new(2.0) + t6814 / F::new(2.0);
    (t6802, t6816)
}
