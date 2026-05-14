//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1267/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1267<F: Float>(t639: F, t9099: F, t1676: F, t1535: F, t1536: F, t16548: F, t16550: F, t16563: F, t24064: F, t24528: F, t24529: F, t24530: F, t2536: F, t2718: F, t3401: F, t5191: F, t568: F, t637: F, t6806: F, t8751: F) -> (F,) {
    let t24934 = t9099 * t639;
    let t24941 = t9099 * t1676;
    let t24951 = 6.0 * t1535 * t24934 * t568 - 6.0 * t1535 * t6806 * t8751 + 12.0 * t1536 * t24064 * t2718 - 2.0 * t24941 * t2536 * t637 + 6.0 * t2718 * t3401 * t5191 + t16548 + t16550 - t16563 + t24528 + t24529 - t24530;
    (t24951,)
}
