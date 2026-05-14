//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1193/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1193<F: Float>(t10375: F, t1641: F, t1445: F, t25556: F, t574: F, t874: F, t2293: F, t7980: F, t2859: F, t31153: F, t2877: F, t6777: F, t2441: F, t8072: F, t8063: F, t26279: F, t895: F) -> (F, F, F, F, F, F, F, F) {
    let t34070 = 0.92023022289409799224e1 * t1641 * t10375;
    let t34074 = 0.46011511144704899612e1 * t574 * t1445 * t25556 * t874;
    let t34078 = 0.92023022289409799224e1 * t574 * t1445 * t7980 * t2293;
    let t34087 = 0.10725146985555128001e1 * t2859 * t31153;
    let t34092 = 0.35750489951850426669e0 * t6777 * t2877;
    let t34094 = 0.71500979903700853338e0 * t2441 * t8072;
    let t34096 = 0.47667319935800568892e0 * t2441 * t8063;
    let t34098 = 0.47667319935800568892e0 * t895 * t26279;
    (t34070, t34074, t34078, t34087, t34092, t34094, t34096, t34098)
}
