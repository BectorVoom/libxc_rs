//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 966/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk966<F: Float>(t29586: F, t29589: F, t29591: F, t29595: F, t29598: F, t29601: F, t29603: F, t29607: F, t29609: F, t29611: F, t29614: F, t29618: F, t29620: F, t29622: F, t29624: F) -> F {
    let t30115 = -F::cast_from(0.101171875e-1_f64) * t29586 - F::new(0.15e1) * t29589 + F::cast_from(0.32375000000000000001e0_f64) * t29591 + F::cast_from(0.27777777777777777777e-1_f64) * t29595 + F::cast_from(0.13489583333333333333e-1_f64) * t29598 - F::new(0.161875e0) * t29601 + F::new(0.1875e0) * t29603 - F::new(0.5625e0) * t29607 - F::cast_from(0.32375000000000000001e0_f64) * t29609 - F::cast_from(0.40468749999999999999e-1_f64) * t29611 + F::cast_from(0.16666666666666666666e0_f64) * t29614 + F::new(0.60703125e-1) * t29618 - F::new(1.0) * t29620 - F::cast_from(0.13489583333333333333e-1_f64) * t29622 + F::new(0.15e1) * t29624;
    t30115
}
