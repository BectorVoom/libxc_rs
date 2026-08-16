//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2246/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246<F: Float>(t1920: F, t25766: F, t968: F, t23384: F, t25739: F, t11010: F, t12652: F, t14552: F, t14555: F, t1603: F, t1956: F, t23327: F, t23329: F, t23571: F, t25423: F, t25429: F, t25430: F, t25743: F, t25755: F, t25767: F, t3020: F, t3169: F, t3207: F, t388: F, t50632: F, t6680: F, t6687: F, t6776: F, t6816: F, t7593: F, t7625: F, t986: F) -> F {
    let t89561 = F::cast_from(0.54831135561607547884e-2_f64) * t1920 * t968 * t25766;
    let t89583 = F::cast_from(0.10966227112321509577e-1_f64) * t23384 * t25739;
    let t89590 = t89561 + F::cast_from(4.0_f64) * t14555 * t6776 + t3020 * t7593 * t388 - t25755 * t3207 - t11010 * t7625 + t1603 * t23571 * t388 - F::cast_from(0.43864908449286038306e-1_f64) * t6680 * t25767 + F::cast_from(4.0_f64) * t3169 * t25743 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t23329 * t25423 * t12652 + F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t23329 * t25430 * t12652 + t89583 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t25766 - F::cast_from(2.0_f64) * t14552 * t6816 - t50632 * t1956;
    t89590
}
