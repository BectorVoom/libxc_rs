//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 790/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk790(t666: f64, t678: f64, t671: f64, t762: f64, t1989: f64, t230: f64, t1985: f64, t226: f64, t1913: f64, t20: f64, t2004: f64, t163: f64, t169: f64, t684: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5945 = t666 * t678;
    let t5948 = 0.11033703703703703703e-2_f64 * t762 * t671;
    let t5949 = t1989 * t230;
    let t5952 = 4.0_f64 * t226 * t1985;
    let t5953 = t1913 * t20;
    let t5954 = t5953 * t2004;
    let t5969 = t169 * t784 * t684 * t163;
    (t5945, t5948, t5949, t5952, t5954, t5969)
}
