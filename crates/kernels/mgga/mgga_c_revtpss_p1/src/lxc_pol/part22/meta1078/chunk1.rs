//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3861/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3861<F: Float>(t125: F, t21969: F, t1399: F, t6883: F, t9816: F, t9818: F, t13999: F, t22271: F, t48919: F, t6869: F, t13716: F, t13944: F, t1872: F, t22096: F, t3889: F, t3934: F, t3936: F, t3944: F, t48508: F, t48510: F, t48595: F, t543: F, t5674: F, t6849: F, t800: F, t9748: F) -> F {
    let t74177 = t125 * t21969;
    let t74184 = t9816 * t9818 * t6883 * t1399;
    let t74186 = t13999 * t22271;
    let t74206 = t9816 * t9818 * t48919 * t6869;
    let t74215 = F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t3936 * t74177 * t1399 + F::cast_from(0.10164000561857065645e-3_f64) * t74184 - F::cast_from(0.40015750243531754508e-2_f64) * t74186 - t9748 * t800 * t6849 * t3889 / F::new(4.0) + t3944 * t800 * t1872 * t13716 / F::new(8.0) + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t3936 * t48595 * t6869 + F::cast_from(0.34299214494455789578e-2_f64) * t3934 * t3936 * t13944 * t22096 + F::cast_from(0.2032800112371413129e-3_f64) * t74206 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t3936 * t5674 * t543 * t13716 - F::cast_from(0.10841600599314203355e-1_f64) * t48508 - F::cast_from(0.80031500487063509016e-1_f64) * t48510;
    t74215
}
