//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1350/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1350<F: Float>(t10375: F, t1641: F, t1445: F, t25556: F, t574: F, t874: F, t2293: F, t7980: F, t2859: F, t31153: F, t2877: F, t6777: F) -> (F, F, F, F, F) {
    let t34070 = F::new(0.92023022289409799224e1) * t1641 * t10375;
    let t34074 = F::new(0.46011511144704899612e1) * t574 * t1445 * t25556 * t874;
    let t34078 = F::new(0.92023022289409799224e1) * t574 * t1445 * t7980 * t2293;
    let t34087 = F::new(0.10725146985555128001e1) * t2859 * t31153;
    let t34092 = F::new(0.35750489951850426669e0) * t6777 * t2877;
    (t34070, t34074, t34078, t34087, t34092)
}
