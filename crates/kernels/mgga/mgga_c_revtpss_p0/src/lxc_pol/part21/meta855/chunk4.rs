//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3239/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3239<F: Float>(t12584: F, t12587: F, t1832: F, t3798: F, t44126: F, t5023: F, t5501: F, t57846: F, t57849: F, t57851: F, t57853: F, t57856: F, t57860: F, t57863: F, t57907: F, t57911: F) -> F {
    let t60139 = -F::cast_from(6.0_f64) * t12584 * t1832 * t44126 * t5023 + F::cast_from(6.0_f64) * t12587 * t3798 * t5023 * t5501 + t57846 + t57849 + t57851 + t57853 + t57856 + t57860 - t57863 - t57907 + t57911;
    t60139
}
