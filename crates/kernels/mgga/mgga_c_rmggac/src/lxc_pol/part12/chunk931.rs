//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 931/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk931<F: Float>(t1163: F, t1979: F, t1982: F, t2313: F, t2189: F, t3350: F, t8515: F, t8519: F, t1971: F, t236: F, t5564: F, t8517: F, t5567: F, t2160: F, t638: F, t8850: F) -> (F, F, F, F, F) {
    let t41999 = t2313 * t1163 * t1979 * t1982;
    let t42003 = t2189 * t8515 * t3350 * t8519;
    let t42007 = t8517 * t1971 * t236 * t5564;
    let t42011 = t8517 * t1971 * t236 * t5567;
    let t42023 = t638 * t2160 * t8850;
    (t41999, t42003, t42007, t42011, t42023)
}
