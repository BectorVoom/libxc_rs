//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1086/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1086<F: Float>(t1162: F, t17666: F, t2367: F, t55330: F, t9116: F, t9118: F, t12079: F, t3103: F, t5313: F, t1170: F, t18098: F, t2586: F, t16024: F, t4509: F, t1168: F, t17885: F, t871: F) -> (F, F, F, F, F, F) {
    let t55346 = t1162 * t2367 * t17666;
    let t55361 = t9116 * t55330 * t9118;
    let t55364 = t3103 * t12079 * t5313;
    let t55390 = t1170 * t2586 * t18098;
    let t55392 = t16024 * t4509;
    let t55396 = t1168 * t17885 * t871;
    (t55346, t55361, t55364, t55390, t55392, t55396)
}
