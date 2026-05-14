//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1110/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1110<F: Float>(t11627: F, t12434: F, t12560: F, t12563: F, t12565: F, t12569: F, t12573: F, t12577: F, t12706: F, t41108: F, t41109: F, t41110: F, t41111: F, t41112: F, t41113: F, t42438: F, t42815: F, t42867: F, t42924: F, t42973: F, t43761: F, t43806: F, t44001: F, t8: F) -> (F,) {
    let t44006 = -t12434 + t41108 + t11627 - t41109 + t12706 + t8 * (t42438 + t42815 + t42867 + t42924 + t42973 + t43761 + t43806 + t44001) + t12560 - t41110 + t41111 + t41112 - t12563 + t12565 + t12569 + t12573 + t12577 + t41113;
    (t44006,)
}
