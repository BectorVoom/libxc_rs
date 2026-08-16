//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1088/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1088<F: Float>(t2068: F, t7342: F, t8480: F, t30782: F, t34879: F, t34883: F, t34887: F, t34891: F, t34894: F, t34896: F, t34897: F, t34901: F, t34905: F, t34909: F, t34913: F, t34916: F, t34920: F, t34923: F, t34926: F, t34929: F) -> F {
    let t34933 = t2068 * t8480 * t7342;
    let t34935 = F::cast_from(0.42874018118069736972e-3_f64) * t34879 - F::cast_from(0.22921875e-1_f64) * t34883 - F::cast_from(0.4584375e-1_f64) * t34887 - F::cast_from(0.22921875e-1_f64) * t34891 + t34894 + t34896 - F::cast_from(0.65369791666666666667e-1_f64) * t34897 + F::cast_from(0.22921875e0_f64) * t34901 - t34905 / F::cast_from(16.0_f64) - F::cast_from(0.916875e-1_f64) * t34909 - F::cast_from(0.4584375e-1_f64) * t34913 - F::cast_from(0.4584375e-1_f64) * t34916 - F::cast_from(0.4584375e-1_f64) * t34920 - F::cast_from(0.4584375e-1_f64) * t34923 - F::cast_from(0.916875e-1_f64) * t34926 - F::cast_from(0.4584375e-1_f64) * t34929 - F::cast_from(0.916875e-1_f64) * t30782 + F::cast_from(0.42874018118069736972e-3_f64) * t34933;
    t34935
}
