//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1146/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1146<F: Float>(t174: F, t507: F, t435: F, t495: F, t930: F, t1298: F, t407: F, t1150: F, t1165: F, t1173: F, t1460: F, t1524: F, t1532: F, t15690: F, t15695: F, t15814: F, t1849: F, t301: F, t372: F, t4255: F, t4256: F, t4257: F, t4261: F, t4263: F, t4593: F, t5164: F, t5544: F, t5549: F, t5651: F, t5693: F, t839: F, t8927: F, t922: F) -> (F, F) {
    let t20555 = t507 * t174;
    let t20559 = t507 * t435;
    let t20590 = t930 * t495;
    let t20595 = t407 * t1298;
    let t20600 = -t1150 * t4593 * t5164 / F::new(8.0) - t4255 * t20555 * t4257 / F::new(4.0) - t4261 * t20559 * t4263 / F::new(6.0) - t15690 * t8927 * t1460 * t1524 / F::new(4.0) - t4255 * t15695 * t5693 / F::new(4.0) - t4255 * t4256 * t5544 * t301 / F::new(4.0) - t4255 * t4256 * t5549 * t301 / F::new(4.0) - t4255 * t4256 * t1849 * t839 / F::new(8.0) - t4255 * t4256 * t5651 * t372 / F::new(8.0) + t15814 * t4256 * t1849 * t922 / F::new(2.0) + F::new(0.17149607247227894789e-2) * t1173 * t1165 * t1532 * t20590 + F::new(0.34299214494455789578e-2) * t1173 * t1165 * t1532 * t20595;
    (t20595, t20600)
}
