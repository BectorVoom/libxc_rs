//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1386/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1386<F: Float>(t55556: F, t55557: F, t57082: F, t57086: F, t57088: F, t57090: F, t57092: F, t57094: F, t57096: F, t57098: F, t57100: F, t57102: F, t57104: F) -> F {
    let t58709 = -t57082 / F::cast_from(384.0_f64) + t57086 / F::cast_from(24.0_f64) - t57088 / F::cast_from(12.0_f64) - t57090 / F::cast_from(48.0_f64) - t57092 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t57094 + t57096 / F::cast_from(48.0_f64) + t57098 / F::cast_from(24.0_f64) + t55556 + t57100 / F::cast_from(48.0_f64) - t57102 / F::cast_from(48.0_f64) - t55557 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57104;
    t58709
}
