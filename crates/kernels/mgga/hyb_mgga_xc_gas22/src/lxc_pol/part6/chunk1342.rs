//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1342/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1342<F: Float>(t24556: F, t24559: F, t24562: F, t24658: F, t24661: F, t24664: F, t24667: F, t24670: F, t24673: F, t28907: F, t28917: F, t28919: F) -> F {
    let t29301 = F::new(0.10589175e2) * t28907 + F::new(0.6311625e0) * t28917 + F::new(0.3529725e1) * t28919 - F::cast_from(0.32136222222222222223e1_f64) * t24556 + F::cast_from(0.27545333333333333334e1_f64) * t24559 - F::new(0.103295e1) * t24562 + F::cast_from(0.13892666666666666667e1_f64) * t24658 + F::cast_from(0.13892666666666666667e1_f64) * t24661 - F::cast_from(0.18523555555555555555e1_f64) * t24664 - F::new(0.41678e0) * t24667 - F::new(0.83356e0) * t24670 - F::new(0.41678e0) * t24673;
    t29301
}
