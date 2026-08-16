//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 744/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk744(t6480: f64, t6484: f64, t6488: f64, t6492: f64, t6823: f64, t6827: f64, t6829: f64, t6832: f64, t6834: f64, t6836: f64, t6840: f64, t1879: f64, t2193: f64, t2238: f64, t3308: f64, t616: f64, t6312: f64, t6318: f64, t6321: f64, t6324: f64, t6328: f64, t6330: f64, t6332: f64, t6333: f64, t6335: f64, t6337: f64, t6359: f64, t6437: f64, t6449: f64, t6457: f64, t6614: f64, t6619: f64, t6621: f64, t6623: f64, t6625: f64, t6627: f64, t6628: f64, t6634: f64, t6638: f64, t6640: f64, t6644: f64, t6697: f64, t6702: f64, t6704: f64, t6709: f64, t6711: f64, t6737: f64, t6774: f64, t7168: f64, t758: f64, param_c1: f64) -> f64 {
    let t7169 = -t6480 - t6484 + t6488 - t6823 + t6827 + t6829 + t6832 + t6834 + t6836 + t6492 - t6840;
    let t7173 = param_c1 * (3.0_f64 / 2.0_f64 * t2238 * t758 + t7169 + t6774 + 3.0_f64 / 2.0_f64 * t6702 - t6638 + t6359 - t6332 - 7.0_f64 * t6333 - t6709 + t6737 - t6627 + t6318 - t6321 + t7168 + t6711 + t6614 + t6457 + t6324 - t6328 + t6330 - t6623 + t6625 + t6449 + 0.23260393291413087447e-1_f64 * t1879 * t6312 * t616 - 0.77534644304710291488e-2_f64 * t3308 * t6628 * t2193 - 0.23260393291413087447e-1_f64 * t1879 * t6704 * t616 + t6697 - t6619 + t6621 - t6634 - t6640 - t6644 - t6437 - 7.0_f64 / 2.0_f64 * t6335 - 7.0_f64 * t6337);
    t7173
}
