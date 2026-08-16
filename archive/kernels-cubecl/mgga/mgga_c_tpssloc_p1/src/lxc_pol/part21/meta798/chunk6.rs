//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2778/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778<F: Float>(t16969: F, t9638: F, t13258: F, t16928: F, t41385: F, t5587: F, t16673: F, t2629: F, t58181: F, t842: F, t13173: F, t13177: F, t13222: F, t13231: F, t13262: F, t16836: F, t16872: F, t16985: F, t20981: F, t2379: F, t2623: F, t2635: F, t2643: F, t2681: F, t40971: F, t41096: F, t4167: F, t4178: F, t4236: F, t47012: F, t47027: F, t47262: F, t47285: F, t5527: F, t5591: F, t5628: F, t58139: F, t820: F, t843: F, t847: F, t849: F, t9990: F) -> F {
    let t58791 = t9638 * t16969;
    let t58797 = t13258 * t16928;
    let t58809 = t41385 * t5587;
    let t58811 = t16673 * t2629;
    let t58834 = t58181 * t842;
    let t58837 = -F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58791 + t2643 * t13222 * t47262 * t5591 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t58797 - t4178 * t13222 * t47262 * t20981 / F::cast_from(64.0_f64) + t13262 * t13222 * t47285 * t47012 / F::cast_from(64.0_f64) - t16836 * t13231 / F::cast_from(96.0_f64) + t41096 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t58809 + t58811 * t2635 / F::cast_from(1536.0_f64) + F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t843 * t40971 * t820 * t5527 * t2379 - t13177 * t4236 / F::cast_from(768.0_f64) - t4167 * t13173 / F::cast_from(1536.0_f64) - t16872 * t2681 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t47027 - t9990 * t5628 / F::cast_from(768.0_f64) - t2623 * t16985 / F::cast_from(384.0_f64) - t843 * t847 * t820 * t58139 / F::cast_from(768.0_f64) - t58834 * t849 / F::cast_from(384.0_f64);
    t58837
}
