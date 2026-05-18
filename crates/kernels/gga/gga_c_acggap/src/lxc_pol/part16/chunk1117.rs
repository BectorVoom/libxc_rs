//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1117/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1117<F: Float>(t142: F, t6379: F, t8806: F, t6383: F, t1318: F, t507: F, t7436: F, t6388: F, t5906: F, t30725: F, t30729: F, t34746: F, t34754: F, t37233: F, t39438: F, t39442: F, t39447: F, t39451: F, t39454: F, t39458: F, t39462: F) -> F {
    let t39465 = t8806 * t142 * t6379;
    let t39468 = t8806 * t142 * t6383;
    let t39471 = t7436 * t507 * t1318;
    let t39474 = t8806 * t142 * t6388;
    let t39477 = t7436 * t142 * t5906;
    let t39479 = -t34746 + F::new(0.52413487149340253447e-3) * t39438 + t37233 + F::new(0.31448092289604152068e-3) * t39442 + t34754 + F::new(0.15724046144802076034e-2) * t30725 + t30729 - F::new(0.15724046144802076034e-2) * t39447 + F::new(0.28582678745379824648e-3) * t39451 + F::new(0.42874018118069736972e-3) * t39454 + F::new(0.62896184579208304136e-3) * t39458 + F::new(0.62896184579208304136e-3) * t39462 - t39465 / F::new(16.0) + t39468 / F::new(8.0) + t39471 / F::new(24.0) + t39474 / F::new(16.0) + t39477 / F::new(48.0);
    t39479
}
