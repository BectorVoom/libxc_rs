//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 826/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk826<F: Float>(t1676: F, t4753: F, t1670: F, t4787: F, t10690: F, t591: F, t10696: F, t1965: F, t5365: F, t1961: F, t5397: F, t240: F, t4998: F, t5493: F, t2013: F, t10463: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12090 = t4753 * t1676;
    let t12095 = t1670 * t4787;
    let t12098 = t591 * t10690;
    let t12105 = t591 * t10696;
    let t12109 = t5365 * t1965;
    let t12114 = t1961 * t5397;
    let t12131 = t240 * t4753;
    let t12162 = t4998 * t5493;
    let t12163 = t2013 * t12162;
    let t12169 = t786 * t10463;
    (t12090, t12095, t12098, t12105, t12109, t12114, t12131, t12163, t12169)
}
