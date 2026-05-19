//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 790/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk790<F: Float>(t666: F, t678: F, t671: F, t762: F, t1989: F, t230: F, t1985: F, t226: F, t1913: F, t20: F, t2004: F, t163: F, t169: F, t684: F, t784: F) -> (F, F, F, F, F, F) {
    let t5945 = t666 * t678;
    let t5948 = F::cast_from(0.11033703703703703703e-2_f64) * t762 * t671;
    let t5949 = t1989 * t230;
    let t5952 = F::new(4.0) * t226 * t1985;
    let t5953 = t1913 * t20;
    let t5954 = t5953 * t2004;
    let t5969 = t169 * t784 * t684 * t163;
    (t5945, t5948, t5949, t5952, t5954, t5969)
}
