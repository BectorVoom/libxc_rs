//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3272/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3272(t1395: f64, t1879: f64, t22223: f64, t22229: f64, t22237: f64, t22240: f64, t22246: f64, t225: f64, t22936: f64, t541: f64, t543: f64, t5644: f64, t5652: f64, t5655: f64, t6832: f64, t6837: f64, t6840: f64, t73: f64, t85892: f64, t85901: f64, t85907: f64, t85915: f64, t85927: f64, t85977: f64, t85988: f64, t85995: f64, t86052: f64) -> f64 {
    let t86054 = (-(t85892 + t85901 + t85907 + t85915 + t85927 + t85977 + t85988 + t85995) * t225 * t541 + 3.0_f64 * t22936 * t1395 + 9.0_f64 * t22223 * t1879 - 36.0_f64 * t6832 * t73 * t5652 + 9.0_f64 * t6832 * t5655 - 36.0_f64 * t5644 * t6837 + 180.0_f64 * t22229 * t22237 - 72.0_f64 * t22229 * t22240 + 9.0_f64 * t5644 * t6840 - 36.0_f64 * t22229 * t22246 + t86052) * t543;
    t86054
}
