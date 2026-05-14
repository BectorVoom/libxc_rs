//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 838/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk838<F: Float>(t1654: F, t761: F, t2061: F, t2049: F, t597: F, t4791: F, t4794: F, t4798: F, t4806: F, t4967: F, t4969: F, t4972: F, t4975: F, t4977: F, t4979: F, t4981: F, t4984: F, t4988: F, t4992: F) -> (F, F, F, F, F) {
    let t5998 = t1654 * t761;
    let t5999 = t2061 * t5998;
    let t6001 = t597 * t2049;
    let t6002 = t2061 * t6001;
    let t6004 = -t4967 - t4969 - t4972 + t4975 + t4977 + t4979 - t4981 + t4984 - t4791 + t4794 + t4798 - t4806 + t4988 + t4992 - 0.4051561992e0 * t5999 - 0.2025780996e0 * t6002;
    (t5998, t5999, t6001, t6002, t6004)
}
