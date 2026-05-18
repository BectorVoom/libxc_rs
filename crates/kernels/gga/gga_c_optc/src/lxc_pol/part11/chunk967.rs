//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 967/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk967<F: Float>(t17723: F, t894: F, t1506: F, t19: F, t4356: F, t15236: F, t4305: F, t5268: F, t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8885: F) -> (F, F, F, F, F, F) {
    let t17724 = t894 * t17723;
    let t17727 = t19 * t1506;
    let t17728 = t17727 * t4356;
    let t17729 = t15236 * t17728;
    let t17733 = F::new(0.17544670192365612213e1) * t4305 * t5268;
    let t17744 = -t8885 - F::new(0.23744444444444444444e-1) * t11671 + F::new(0.11872222222222222222e-1) * t14885 - F::new(0.35616666666666666666e-1) * t14887 + F::new(0.17808333333333333333e-1) * t14889 - F::new(0.19787037037037037037e-1) * t17338 + F::new(0.71233333333333333332e-1) * t17342 - F::new(0.35616666666666666666e-1) * t17346 - F::new(0.10685e0) * t17350 + F::new(0.10685e0) * t17354 - F::new(0.17808333333333333333e-1) * t17358;
    (t17724, t17727, t17728, t17729, t17733, t17744)
}
