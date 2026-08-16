//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3520/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520<F: Float>(t4866: F, t906: F, t15689: F, t15691: F, t16052: F, t19973: F, t3162: F, t42795: F, t54387: F, t54407: F, t54414: F, t54432: F, t54435: F, t54438: F, t54440: F, t54443: F, t54446: F, t54469: F) -> (F, F) {
    let t66667 = t906 * t4866;
    let t66682 = F::cast_from(0.19055119163586549765e-3_f64) * t54387 + F::cast_from(0.95275595817932748826e-4_f64) * t42795 + F::cast_from(0.57165357490759649296e-3_f64) * t54407 - F::cast_from(0.57165357490759649296e-3_f64) * t15689 * t15691 * t3162 * t66667 + F::cast_from(0.1270341277572436651e-3_f64) * t54414 - F::cast_from(0.7622047665434619906e-3_f64) * t54432 - F::cast_from(0.3811023832717309953e-3_f64) * t54435 - F::cast_from(0.19055119163586549765e-2_f64) * t54438 + F::cast_from(0.6351706387862183255e-3_f64) * t54440 + F::cast_from(0.31758531939310916275e-3_f64) * t54443 + F::cast_from(0.84689418504829110067e-3_f64) * t54446 + F::cast_from(0.11433071498151929859e-2_f64) * t54469 - F::cast_from(0.91464571985215438873e-2_f64) * t16052 * t19973;
    (t66667, t66682)
}
