//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 740/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk740<F: Float>(t12829: F, t539: F, t1587: F, t398: F, t1390: F, t1588: F, t12951: F, t13614: F, t397: F, t535: F, t1609: F, t551: F) -> (F, F, F, F, F, F) {
    let t14935 = t539 * t12829;
    let t14961 = t1587 * t1587;
    let t14962 = F::new(1.0) / t14961;
    let t14963 = t398 * t14962;
    let t14978 = t1588 * t1390;
    let t14995 = t539 * t12951;
    let t15050 = t397 * t13614 * t539;
    let t15052 = F::cast_from(0.9994882620098509563e-2_f64) * t535 * t15050;
    let t15092 = t1609 * t1609;
    let t15093 = F::new(1.0) / t15092;
    let t15094 = t551 * t15093;
    (t14935, t14963, t14978, t14995, t15052, t15094)
}
