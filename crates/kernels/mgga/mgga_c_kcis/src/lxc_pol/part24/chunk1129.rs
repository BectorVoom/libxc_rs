//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1129/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1129<F: Float>(t26558: F, t26517: F, t26417: F, t26632: F, t782: F, t826: F, t26390: F, t31271: F, t2585: F, t740: F, t7617: F, t9181: F) -> (F, F, F, F, F, F, F) {
    let t91781 = F::cast_from(3.0_f64) * t26558;
    let t91785 = F::cast_from(6.0_f64) * t26517;
    let t91786 = F::cast_from(6.0_f64) * t26417;
    let t91789 = t26632 * t782;
    let t91791 = F::cast_from(3.0_f64) * t91789 * t826;
    let t91793 = F::cast_from(18.0_f64) * t31271 * t26390;
    let t91794 = t2585 * t740;
    let t91796 = t9181 * t7617;
    (t91781, t91785, t91786, t91791, t91793, t91794, t91796)
}
