//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 797/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk797<F: Float>(t3644: F, t967: F, t100: F, t10050: F, t10053: F, t1563: F, t10: F, t10069: F, t10072: F, t10075: F, t10078: F, t496: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5776: F) -> (F, F, F, F, F, F, F) {
    let t12898 = t3644 * t967;
    let t12899 = t12898 * t100;
    let t12906 = F::cast_from(0.2923025e1_f64) * t10050;
    let t12907 = F::cast_from(0.14615125e1_f64) * t10053;
    let t12912 = t1563 * t12898;
    let t12913 = t10 * t12912;
    let t12916 = -t5749 - t5751 + t5753 - t5755 - t5759 - t12906 + t12907 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10069 + t10072 / F::cast_from(2.0_f64) - F::cast_from(0.881424e1_f64) * t10075 + F::cast_from(0.220356e1_f64) * t10078 - t5776 - F::cast_from(6.0_f64) * t496 * t12913;
    (t12898, t12899, t12906, t12907, t12912, t12913, t12916)
}
