//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 880/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk880<F: Float>(t36042: F, t871: F, t1466: F, t33808: F, t35795: F, t35799: F, t35802: F, t35810: F, t35814: F, t35817: F, t36003: F, t36005: F, t36007: F, t36009: F, t36013: F, t36017: F, t6216: F, t6963: F, t6967: F, t6972: F, t7028: F, t7581: F, t7587: F, t7618: F) -> (F, F) {
    let t36043 = t871 * t36042;
    let t36047 = -t1466 * t35795 / F::new(3.0) + t1466 * t35799 - F::new(2.0) / F::new(3.0) * t1466 * t35802 - t7581 * t6972 / F::new(3.0) - t33808 * t6967 / F::new(18.0) - t6216 * t35810 / F::new(18.0) + t6216 * t35814 / F::new(9.0) + F::new(4.0) * t35817 + F::new(2.0) * t36003 - F::new(4.0) * t36005 - F::new(2.0) * t36007 - F::new(4.0) * t36009 - F::new(2.0) / F::new(3.0) * t1466 * t36013 + t1466 * t36017 / F::new(6.0) + t7581 * t7028 / F::new(6.0) - t6963 * t7587 / F::new(3.0) - F::new(2.0) * t36043 + t6963 * t7618 / F::new(3.0);
    (t36043, t36047)
}
