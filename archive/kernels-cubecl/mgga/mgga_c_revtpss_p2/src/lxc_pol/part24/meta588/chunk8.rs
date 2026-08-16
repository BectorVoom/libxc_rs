//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1844/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1844<F: Float>(t6922: F, t1868: F, t1907: F, t198: F, t21937: F, t39989: F, t4139: F, t4147: F, t47084: F, t47086: F, t532: F, t5536: F, t5541: F, t6781: F, t6816: F, t73499: F, t86819: F, t86825: F, t86828: F, t91984: F, t91985: F, t92013: F, t92014: F, t92015: F, t92016: F) -> F {
    let t92482 = t6922 * t6922;
    let t92490 = -F::cast_from(3.0_f64) * t198 * t4147 * t532 * t92482 + F::cast_from(24.0_f64) * t1868 * t4139 * t86828 + F::cast_from(24.0_f64) * t1868 * t5536 * t86819 - F::cast_from(4.0_f64) * t1907 * t5541 * t86825 + F::cast_from(18.0_f64) * t21937 * t4139 * t6816 + F::cast_from(12.0_f64) * t5541 * t6781 * t73499 - t39989 - t47084 - t47086 - t91984 - t91985 + t92013 - t92014 + t92015 + t92016;
    t92490
}
