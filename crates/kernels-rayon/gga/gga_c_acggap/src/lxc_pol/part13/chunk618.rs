//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 618/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk618(t1524: f64, t360: f64, t1083: f64, t398: f64, t372: f64, t1459: f64, t1173: f64, t1180: f64, t3396: f64, t418: f64, t4695: f64, t4699: f64, t4701: f64, t4705: f64, t4708: f64, t4713: f64, t4716: f64, t4722: f64, t4724: f64, t4728: f64, t4732: f64, t4735: f64, t4737: f64, t4742: f64, t4745: f64, t4747: f64, t4748: f64, t4750: f64, t4754: f64) -> (f64, f64, f64, f64, f64) {
    let t4757 = t1524 * t360;
    let t4759 = t398 * t1083 * t4757;
    let t4762 = t1524 * t372;
    let t4764 = t398 * t1459 * t4762;
    let t4767 = 0.85748036236139473944e-3_f64 * t1180 * t4695 + t4699 + 0.17149607247227894789e-2_f64 * t1173 * t4701 - t4705 + 0.85748036236139473944e-3_f64 * t1173 * t4708 - 0.42874018118069736972e-3_f64 * t1180 * t4713 - 0.80031500487063509015e-2_f64 * t4716 + t4722 - 0.34299214494455789578e-2_f64 * t418 * t4724 - 0.17149607247227894789e-2_f64 * t418 * t4728 + 0.68598428988911579156e-2_f64 * t3396 * t4732 + 0.51448821741683684367e-2_f64 * t4735 * t4737 - 0.25724410870841842184e-2_f64 * t4742 + t4745 - t4747 + 0.25724410870841842184e-2_f64 * t4748 + 0.85748036236139473944e-3_f64 * t4750 + 0.34299214494455789578e-2_f64 * t418 * t4754 - 0.17149607247227894789e-2_f64 * t418 * t4759 + 0.25724410870841842184e-2_f64 * t418 * t4764;
    (t4757, t4759, t4762, t4764, t4767)
}
