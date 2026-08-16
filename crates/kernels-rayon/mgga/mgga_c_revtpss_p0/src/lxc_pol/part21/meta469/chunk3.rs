//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2023/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2023(t10824: f64, t10826: f64, t10833: f64, t10838: f64, t10842: f64, t10846: f64, t10853: f64, t10855: f64, t10859: f64, t10881: f64, t10885: f64, t10888: f64) -> f64 {
    let t14889 = -t10824 + t10826 - 0.12705000702321332056e-4_f64 * t10833 - 0.57165357490759649296e-4_f64 * t10838 - 0.12705000702321332056e-4_f64 * t10842 + 0.27104001498285508387e-3_f64 * t10846 + 0.25410001404642664112e-4_f64 * t10853 + 0.10003937560882938627e-2_f64 * t10855 - 0.20007875121765877254e-2_f64 * t10859 + 0.10003937560882938627e-2_f64 * t10881 - t10885 + 0.2032800112371413129e-4_f64 * t10888;
    t14889
}
