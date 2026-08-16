//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1420/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1420<F: Float>(t113: F, t121958: F, t122082: F, t121231: F, t121233: F, t121234: F, t121237: F, t121240: F, t121253: F, t121254: F, t2039: F, t2075: F, t2314: F, t24983: F, t25958: F, t26098: F, t31734: F, t33350: F, t4034: F, t652: F, t7042: F, t7458: F) -> F {
    let t122084 = t113 * (t121958 + t122082);
    let t122085 = -F::cast_from(2.0_f64) * t2039 * t25958 * t652 - t2075 * t26098 - F::cast_from(2.0_f64) * t2314 * t33350 - F::cast_from(2.0_f64) * t24983 * t7042 - F::cast_from(2.0_f64) * t31734 * t7458 - F::cast_from(2.0_f64) * t33350 * t4034 - t121231 - t121233 - t121234 - t121237 - t121240 - t121253 - t121254 - t122084;
    t122085
}
