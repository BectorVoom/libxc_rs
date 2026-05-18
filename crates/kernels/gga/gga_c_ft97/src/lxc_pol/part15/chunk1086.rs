//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1086/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1086<F: Float>(t4417: F, t4668: F, t12680: F, t13208: F, t17198: F, t1901: F, t20764: F, t20768: F, t2210: F, t3434: F, t3440: F, t41269: F, t4454: F, t4458: F, t4733: F, t77196: F, t77198: F, t77214: F, t85401: F, t87009: F, t9133: F, t9144: F) -> (F, F) {
    let t87534 = t4417 * t4668;
    let t87552 = F::new(8.0) / F::new(3.0) * t1901 * t9144 * t4458 * t4733 - F::new(8.0) / F::new(3.0) * t1901 * t13208 * t87009 - F::new(8.0) / F::new(9.0) * t1901 * t41269 * t4454 * t4733 - F::new(4.0) / F::new(3.0) * t1901 * t2210 * t17198 * t4458 + F::new(8.0) / F::new(3.0) * t1901 * t9133 * t3434 * t87534 + F::new(4.0) / F::new(3.0) * t1901 * t12680 * t20764 + F::new(8.0) / F::new(3.0) * t1901 * t12680 * t20768 - F::new(4.0) * t1901 * t2210 * t3440 * t85401 + F::new(8.0) / F::new(27.0) * t77196 + F::new(8.0) / F::new(9.0) * t77198 - F::new(4.0) / F::new(9.0) * t77214;
    (t87534, t87552)
}
