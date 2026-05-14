//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1278/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1278<F: Float>(t34206: F, t5074: F, t1944: F, t2454: F, t1799: F, t9680: F, t1872: F, t1869: F, t33027: F, t34086: F, t4811: F, t2571: F, t34159: F, t5063: F, t32995: F, t34122: F) -> (F, F, F, F, F, F, F, F) {
    let t116133 = t5074 * t34206;
    let t116137 = t1944 * t2454;
    let t116139 = t1799 * t116137 * t9680;
    let t116145 = t1872 * t2454;
    let t116147 = t1869 * t116145 * t33027;
    let t116149 = t4811 * t34086;
    let t116150 = 0.22109259259259259258e-2 * t116149;
    let t116153 = t1869 * t34159 * t2571 * t5063;
    let t116156 = 0.69444444444444444446e-2 * t34122 * t32995;
    (t116133, t116139, t116145, t116147, t116149, t116150, t116153, t116156)
}
