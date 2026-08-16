//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 750/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk750<F: Float>(t11076: F, t1808: F, t3119: F, t91: F, t1767: F, t8345: F, t965: F, t1766: F, t3157: F, t473: F, t11416: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t8455: F) -> (F, F, F, F) {
    let t11781 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11076;
    let t11783 = t91 * t3119 * t1808;
    let t11787 = t91 * t8345 * t965 * t1767;
    let t11789 = t1766 * t3157;
    let t11791 = t91 * t11789 * t473;
    let t11798 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11416;
    let t11799 = -t11781 - t8455 - t11783 / F::cast_from(12.0_f64) + t11787 / F::cast_from(8.0_f64) - t11791 / F::cast_from(6.0_f64) - t11395 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11399 + F::cast_from(22.0_f64) / F::cast_from(27.0_f64) * t11404 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11408 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11413 - t11798;
    (t11783, t11787, t11791, t11799)
}
