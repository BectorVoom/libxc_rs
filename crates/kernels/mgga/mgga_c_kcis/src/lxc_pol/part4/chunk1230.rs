//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1230/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1230<F: Float>(t12281: F, t15874: F, t4160: F, t1396: F, t4161: F, t4164: F, t5440: F, t5426: F, t5661: F, t12240: F, t1307: F, t5676: F) -> (F, F, F, F, F) {
    let t15875 = t12281 * t15874;
    let t15876 = t4160 * t15875;
    let t15878 = t4161 * t1396;
    let t15879 = t5440 * t4164;
    let t15880 = t15878 * t15879;
    let t15881 = t4160 * t15880;
    let t15883 = t5426 * t4164;
    let t15884 = t15878 * t15883;
    let t15885 = t5661 * t15884;
    let t15887 = t12240 * t1396;
    let t15888 = t5676 * t1307;
    (t15876, t15881, t15885, t15887, t15888)
}
