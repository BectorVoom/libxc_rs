//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1376/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1376(t116702: f64, t116722: f64, t116735: f64, t116865: f64, t114838: f64, t114841: f64, t114844: f64, t114847: f64, t114850: f64, t114853: f64, t114865: f64, t114871: f64, t114873: f64, t114875: f64, t114877: f64, t114879: f64, t114882: f64, t1918: f64, t2170: f64, t25063: f64, t25066: f64, t25069: f64, t30985: f64, t573: f64, t6945: f64, t6948: f64, t8245: f64, param_d: f64) -> (f64, f64) {
    let t116867 = t116702 + t116722 + t116735 + t116865;
    let t116876 = t116867 * t573 * param_d + 9.0_f64 * t1918 * t30985 + 6.0_f64 * t2170 * t25063 + 18.0_f64 * t2170 * t25066 + 3.0_f64 * t2170 * t25069 + 18.0_f64 * t6945 * t8245 + 9.0_f64 * t6948 * t8245 + t114838 + t114841 + t114844 + t114847 + t114850 + t114853 + t114865 + t114871 + t114873 + t114875 + t114877 + t114879 + t114882;
    (t116867, t116876)
}
