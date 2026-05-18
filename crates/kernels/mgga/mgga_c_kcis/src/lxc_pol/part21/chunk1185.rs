//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1185/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1185<F: Float>(t26409: F, t26655: F, t26520: F, t26558: F, t26517: F, t26417: F, t26632: F, t782: F, t826: F, t26390: F, t31271: F, t2585: F, t740: F) -> (F, F, F, F, F, F, F, F, F) {
    let t91776 = F::new(6.0) * t26409;
    let t91777 = F::new(3.0) * t26655;
    let t91778 = F::new(3.0) * t26520;
    let t91781 = F::new(3.0) * t26558;
    let t91785 = F::new(6.0) * t26517;
    let t91786 = F::new(6.0) * t26417;
    let t91789 = t26632 * t782;
    let t91791 = F::new(3.0) * t91789 * t826;
    let t91793 = F::new(18.0) * t31271 * t26390;
    let t91794 = t2585 * t740;
    (t91776, t91777, t91778, t91781, t91785, t91786, t91791, t91793, t91794)
}
