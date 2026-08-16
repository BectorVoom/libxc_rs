//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2367/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2367<F: Float>(t13969: F, t13976: F, t3130: F, t1041: F, t14183: F, t10471: F, t47840: F, t10479: F, t10908: F, t4641: F, t10485: F, t10937: F, t10965: F, t14033: F, t14037: F, t14164: F, t2979: F, t42428: F, t42432: F, t4582: F, t4585: F, t4590: F, t47697: F, t48548: F, t48554: F, t973: F) -> (F, F) {
    let t48564 = t3130 * t13969 * t13976;
    let t48567 = t1041 * t13969 * t14183;
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48574 = t4641 * t10908;
    let t48577 = t973 * t2979 * t47697 / F::cast_from(216.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t48548 - t10937 * t14033 / F::cast_from(288.0_f64) - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t10937 * t14037 + t1041 * t4582 * t14164 * t48554 / F::cast_from(256.0_f64) - t10965 * t4585 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t10965 * t4590 + t48564 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t48567 + t48570 * t10485 / F::cast_from(512.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t42428 + t48574 / F::cast_from(1536.0_f64) - t42432 / F::cast_from(6912.0_f64);
    (t48569, t48577)
}
