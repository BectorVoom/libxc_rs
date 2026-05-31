//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 575/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk575<F: Float>(t1896: F, t4811: F, t1901: F, t1862: F, t1871: F, t1895: F, t1869: F, t1691: F, t670: F, t604: F, t1790: F, t667: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4812 = t4811 * t1896;
    let t4814 = t4811 * t1901;
    let t4816 = t1862 * t1871;
    let t4817 = t4816 * sigma2;
    let t4818 = t4817 * t1895;
    let t4819 = t1869 * t4818;
    let t4822 = F::cast_from(1.0_f64) / t1691 / t670;
    let t4823 = t604 * t4822;
    let t4824 = t1790 * t1790;
    let t4825 = t667 * t667;
    (t4812, t4814, t4816, t4817, t4818, t4819, t4822, t4823, t4824, t4825)
}
