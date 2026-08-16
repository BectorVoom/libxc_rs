//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1257/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1257(t1018: f64, t1125: f64, t12267: f64, t12849: f64, t12851: f64, t12854: f64, t12856: f64, t2405: f64, t2406: f64, t2951: f64, t2953: f64, t330: f64, t3517: f64, t3740: f64, t3742: f64, t44609: f64, t44661: f64, t837: f64, t838: f64, t9698: f64) -> f64 {
    let t44684 = (t44609 + t44661) * t330 + t12849 * t837 * t330 + 2.0_f64 * t12267 * t1018 * t330 + 2.0_f64 * t3740 * t2405 * t330 + 2.0_f64 * t12851 * t838 + t3517 * t2951 * t330 + t1125 * t9698 * t330 + t12854 * t838 + t3517 * t2953 * t330 + 2.0_f64 * t3742 * t2406 + t12856 * t838;
    t44684
}
