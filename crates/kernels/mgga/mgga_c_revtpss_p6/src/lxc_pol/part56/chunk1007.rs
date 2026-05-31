//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1007/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1007<F: Float>(t5: F, t2170: F, t7953: F, t8142: F, t8441: F, t8621: F, t33359: F, t33363: F, t33370: F, t33609: F, t33613: F, t33617: F, t33625: F, t8737: F, t8913: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t34485 = t2170 * t7953;
    let t34866 = t8441 * t8142;
    let t34867 = t8621 * t34866;
    let t34873 = piecewise3::<F>(t8, F::cast_from(0.0_f64), F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t33609 * t8913 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t33359 * t33613 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33363 * t33617 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8737 * t34867 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33370 * t33625);
    (t34485, t34866, t34867, t34873)
}
