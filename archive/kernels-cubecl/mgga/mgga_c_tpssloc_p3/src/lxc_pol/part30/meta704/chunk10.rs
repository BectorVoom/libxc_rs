//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2307/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2307<F: Float>(t23384: F, t28638: F, t23665: F, t28605: F, t1610: F, t17876: F, t1953: F, t23346: F, t23685: F, t23696: F, t25706: F, t28641: F, t3200: F, t4615: F, t4684: F, t5677: F, t6687: F, t7622: F, t89151: F, t89156: F, t89158: F) -> F {
    let t100189 = t23384 * t28638;
    let t100193 = t23665 * t28605;
    let t100195 = F::cast_from(0.36554090374405031923e-2_f64) * t6687 * t23696 * t23685 * t5677 - t3200 * t28641 * t4684 + t17876 * t1953 + F::cast_from(2.0_f64) * t1610 * t25706 - F::cast_from(0.97477574331746751795e-2_f64) * t23346 * t28638 + F::cast_from(0.12184696791468343974e-2_f64) * t100189 + t89151 + F::cast_from(2.0_f64) * t4615 * t7622 - F::cast_from(0.54831135561607547883e-2_f64) * t100193 + t89156 + t89158;
    t100195
}
