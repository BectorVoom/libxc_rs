//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1145/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1145<F: Float>(t20489: F, t992: F, t10007: F, t1131: F, t1168: F, t13927: F, t1901: F, t21494: F, t2599: F, t2600: F, t2606: F, t2607: F, t3885: F, t3891: F, t3892: F, t446: F, t4973: F, t5073: F, t51972: F, t67961: F, t68001: F, t729: F, t81162: F, t81164: F) -> (F, F) {
    let t89212 = t20489 * t992;
    let t89221 = -F::new(4.0) / F::new(3.0) * t1901 * t10007 * t4973 * t5073 + F::new(16.0) / F::new(9.0) * t67961 - F::new(8.0) / F::new(27.0) * t68001 - F::new(8.0) * t446 * t729 * t13927 * t21494 + F::new(112.0) / F::new(81.0) * t51972 - F::new(16.0) / F::new(27.0) * t81162 + F::new(4.0) / F::new(9.0) * t81164 + F::new(4.0) / F::new(9.0) * t1901 * t2599 * t2600 * t20489 * t1131 + F::new(4.0) / F::new(9.0) * t1901 * t2606 * t2607 * t20489 * t1168 + F::new(8.0) / F::new(9.0) * t1901 * t2606 * t3885 * t89212 - F::new(8.0) / F::new(27.0) * t1901 * t3891 * t3892 * t89212;
    (t89212, t89221)
}
