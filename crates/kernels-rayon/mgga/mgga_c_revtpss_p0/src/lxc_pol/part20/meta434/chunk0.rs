//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1635/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635(t12625: f64, t458: f64, t456: f64, t225: f64, t480: f64, t3568: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64) -> (f64, f64, f64, f64, f64) {
    let t44841 = 1.0_f64 / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    let t44844 = t44843 * t480;
    let t44845 = t3568 * t3568;
    let t44864 = -0.21950617283950617284e-1_f64 * t43858 - 0.43901234567901234568e-1_f64 * t43862 - 0.11853333333333333334e0_f64 * t43830 - 0.26340740740740740742e-1_f64 * t43865 + 0.39511111111111111112e-1_f64 * t43832 + 0.98777777777777777779e-1_f64 * t43837 - 0.29633333333333333334e-1_f64 * t43871 - 0.39511111111111111112e-1_f64 * t43841 + 0.53340000000000000002e0_f64 * t43845 + 0.88900000000000000002e-1_f64 * t43877 + 0.11853333333333333334e0_f64 * t43849;
    (t44842, t44843, t44844, t44845, t44864)
}
