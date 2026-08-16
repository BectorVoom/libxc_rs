//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1013/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1013<F: Float>(t12514: F, t395: F, t12494: F, t12498: F, t12506: F, t12502: F, t12510: F, t12722: F, t1820: F, t5125: F, t12509: F, t4934: F, t639: F) -> (F, F, F, F, F, F, F, F) {
    let t40954 = t395 * t12514;
    let t40956 = t395 * t12494;
    let t40958 = t395 * t12498;
    let t40960 = t395 * t12506;
    let t40962 = t395 * t12502;
    let t40989 = t395 * t12510;
    let t41039 = t1820 * t5125 * t12722;
    let t41042 = t639 * t4934 * t12509;
    (t40954, t40956, t40958, t40960, t40962, t40989, t41039, t41042)
}
