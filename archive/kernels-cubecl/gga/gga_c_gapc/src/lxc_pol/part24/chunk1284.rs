//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1284/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1284<F: Float>(t35668: F, t35670: F, t35672: F, t35674: F, t35676: F, t35680: F, t35685: F, t35689: F, t35694: F, t35697: F, t35700: F, t35702: F, t35706: F, t35708: F) -> F {
    let t37474 = F::cast_from(0.4858937065101519846e-3_f64) * t35668 + F::cast_from(0.3475929712541504153e-3_f64) * t35670 - F::cast_from(0.23897016773722841052e-3_f64) * t35672 - F::cast_from(0.6180203028898794384e-4_f64) * t35674 + F::cast_from(0.33742618507649443374e-5_f64) * t35676 + F::cast_from(0.98415970647310876507e-6_f64) * t35680 - F::cast_from(0.64585480737297762708e-6_f64) * t35685 + F::cast_from(0.21724560703384400956e-4_f64) * t35689 + F::cast_from(0.2060067676299598128e-5_f64) * t35694 - F::cast_from(0.3475929712541504153e-4_f64) * t35697 - F::cast_from(0.3475929712541504153e-4_f64) * t35700 + F::cast_from(0.28840947468194373792e-3_f64) * t35702 + F::cast_from(0.22892502052879284198e-6_f64) * t35706 - F::cast_from(0.21055393948773252666e-2_f64) * t35708;
    t37474
}
