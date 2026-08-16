//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2532/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2532<F: Float>(t10263: F, t4528: F, t12606: F, t2989: F, t2986: F, t344: F, t43052: F, t4343: F, t2978: F, t4338: F, t697: F, t43053: F, t4514: F) -> (F, F, F, F, F) {
    let t48342 = t10263 * t4528;
    let t48357 = t2989 * t12606;
    let t48373 = t2986 * t43052 * t344 * t4343;
    let t48378 = t2986 * t697 * t2978 * t344 * t4338;
    let t48381 = t2986 * t43053 * t4514;
    (t48342, t48357, t48373, t48378, t48381)
}
