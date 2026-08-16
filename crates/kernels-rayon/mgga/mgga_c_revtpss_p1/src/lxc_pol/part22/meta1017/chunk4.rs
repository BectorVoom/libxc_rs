//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3520/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520(t4866: f64, t906: f64, t15689: f64, t15691: f64, t16052: f64, t19973: f64, t3162: f64, t42795: f64, t54387: f64, t54407: f64, t54414: f64, t54432: f64, t54435: f64, t54438: f64, t54440: f64, t54443: f64, t54446: f64, t54469: f64) -> (f64, f64) {
    let t66667 = t906 * t4866;
    let t66682 = 0.19055119163586549765e-3_f64 * t54387 + 0.95275595817932748826e-4_f64 * t42795 + 0.57165357490759649296e-3_f64 * t54407 - 0.57165357490759649296e-3_f64 * t15689 * t15691 * t3162 * t66667 + 0.1270341277572436651e-3_f64 * t54414 - 0.7622047665434619906e-3_f64 * t54432 - 0.3811023832717309953e-3_f64 * t54435 - 0.19055119163586549765e-2_f64 * t54438 + 0.6351706387862183255e-3_f64 * t54440 + 0.31758531939310916275e-3_f64 * t54443 + 0.84689418504829110067e-3_f64 * t54446 + 0.11433071498151929859e-2_f64 * t54469 - 0.91464571985215438873e-2_f64 * t16052 * t19973;
    (t66667, t66682)
}
