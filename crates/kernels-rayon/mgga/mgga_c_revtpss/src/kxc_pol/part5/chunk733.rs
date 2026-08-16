//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 733/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk733(t4574: f64, t4919: f64, t140: f64, t1655: f64, t1011: f64, t1656: f64, t3115: f64, t3234: f64, t3241: f64, t3245: f64, t4887: f64, t4892: f64, t4896: f64, t4899: f64, t4902: f64, t4907: f64, t4912: f64, t4916: f64) -> (f64, f64, f64) {
    let t4920 = t4919 * t4574;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4928 = -t3241 * t1656 / 108.0_f64 + t1011 * t4887 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t4892 * t4896 - 0.21437009059034868486e-3_f64 * t4899 * t4902 - 0.21437009059034868486e-3_f64 * t3115 * t4907 - 0.21437009059034868486e-3_f64 * t3115 * t4912 - t1011 * t4916 / 144.0_f64 + t1011 * t4920 / 216.0_f64 - 0.76220476654346199061e-3_f64 * t3234 + t4925 / 864.0_f64 + t3245 / 864.0_f64;
    (t4924, t4925, t4928)
}
