//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1086/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1086<F: Float>(t32608: F, t32628: F, t3: F, t112: F, t8919: F, t31277: F, t31279: F, t31282: F, t31284: F, t31287: F, t31940: F, t31942: F, t31944: F, t577: F, t671: F, t8508: F) -> (F, F, F, F) {
    let t32629 = t32608 + t32628;
    let t32630 = t3 * t32629;
    let t32643 = t8919 * t112;
    let t32649 = F::cast_from(0.45e1_f64) * t32629 * t577 + F::cast_from(0.135e2_f64) * t32643 * t671 + F::cast_from(27.0_f64) * t31940 + F::cast_from(54.0_f64) * t31942 + F::cast_from(27.0_f64) * t31944 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    (t32629, t32630, t32643, t32649)
}
