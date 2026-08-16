//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 618/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk618<F: Float>(t1524: F, t360: F, t1083: F, t398: F, t372: F, t1459: F, t1173: F, t1180: F, t3396: F, t418: F, t4695: F, t4699: F, t4701: F, t4705: F, t4708: F, t4713: F, t4716: F, t4722: F, t4724: F, t4728: F, t4732: F, t4735: F, t4737: F, t4742: F, t4745: F, t4747: F, t4748: F, t4750: F, t4754: F) -> (F, F, F, F, F) {
    let t4757 = t1524 * t360;
    let t4759 = t398 * t1083 * t4757;
    let t4762 = t1524 * t372;
    let t4764 = t398 * t1459 * t4762;
    let t4767 = F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t4695 + t4699 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t4701 - t4705 + F::cast_from(0.85748036236139473944e-3_f64) * t1173 * t4708 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t4713 - F::cast_from(0.80031500487063509015e-2_f64) * t4716 + t4722 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t4724 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4728 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t4732 + F::cast_from(0.51448821741683684367e-2_f64) * t4735 * t4737 - F::cast_from(0.25724410870841842184e-2_f64) * t4742 + t4745 - t4747 + F::cast_from(0.25724410870841842184e-2_f64) * t4748 + F::cast_from(0.85748036236139473944e-3_f64) * t4750 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t4754 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4759 + F::cast_from(0.25724410870841842184e-2_f64) * t418 * t4764;
    (t4757, t4759, t4762, t4764, t4767)
}
