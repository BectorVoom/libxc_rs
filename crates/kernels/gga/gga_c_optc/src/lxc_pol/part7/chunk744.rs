//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 744/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk744<F: Float>(t6480: F, t6484: F, t6488: F, t6492: F, t6823: F, t6827: F, t6829: F, t6832: F, t6834: F, t6836: F, t6840: F, t1879: F, t2193: F, t2238: F, t3308: F, t616: F, t6312: F, t6318: F, t6321: F, t6324: F, t6328: F, t6330: F, t6332: F, t6333: F, t6335: F, t6337: F, t6359: F, t6437: F, t6449: F, t6457: F, t6614: F, t6619: F, t6621: F, t6623: F, t6625: F, t6627: F, t6628: F, t6634: F, t6638: F, t6640: F, t6644: F, t6697: F, t6702: F, t6704: F, t6709: F, t6711: F, t6737: F, t6774: F, t7168: F, t758: F, param_c1: F) -> F {
    let t7169 = -t6480 - t6484 + t6488 - t6823 + t6827 + t6829 + t6832 + t6834 + t6836 + t6492 - t6840;
    let t7173 = param_c1 * (F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2238 * t758 + t7169 + t6774 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6702 - t6638 + t6359 - t6332 - F::cast_from(7.0_f64) * t6333 - t6709 + t6737 - t6627 + t6318 - t6321 + t7168 + t6711 + t6614 + t6457 + t6324 - t6328 + t6330 - t6623 + t6625 + t6449 + F::cast_from(0.23260393291413087447e-1_f64) * t1879 * t6312 * t616 - F::cast_from(0.77534644304710291488e-2_f64) * t3308 * t6628 * t2193 - F::cast_from(0.23260393291413087447e-1_f64) * t1879 * t6704 * t616 + t6697 - t6619 + t6621 - t6634 - t6640 - t6644 - t6437 - F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t6335 - F::cast_from(7.0_f64) * t6337);
    t7173
}
