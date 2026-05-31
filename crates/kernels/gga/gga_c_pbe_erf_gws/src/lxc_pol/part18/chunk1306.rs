//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1306/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1306<F: Float>(t11889: F, t1193: F, t13930: F, t3207: F, t39689: F, t53610: F, t53626: F, t53629: F, t53636: F, t53761: F, t54545: F, t56657: F, t56661: F, t56667: F, t56671: F, t56674: F, t56678: F, t56686: F, t827: F, t8793: F, t9283: F) -> F {
    let t56694 = t56657 / F::cast_from(768.0_f64) - t827 * t56661 / F::cast_from(48.0_f64) + t56667 / F::cast_from(384.0_f64) - t827 * t56671 / F::cast_from(96.0_f64) - t56674 / F::cast_from(48.0_f64) - t56678 / F::cast_from(384.0_f64) - t53610 + t8793 * t53761 / F::cast_from(24.0_f64) + t8793 * t54545 / F::cast_from(24.0_f64) - t56686 / F::cast_from(1536.0_f64) + t39689 * t13930 / F::cast_from(48.0_f64) + t53626 + t53629 - t3207 * t9283 * t1193 * t11889 / F::cast_from(8.0_f64) + t53636;
    t56694
}
