//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1166/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1166<F: Float>(t1028: F, t1032: F, t1042: F, t120263: F, t120479: F, t126600: F, t126749: F, t126765: F, t126770: F, t126774: F, t126779: F, t247: F, t3046: F, t3116: F, t31891: F, t31892: F, t31899: F, t31903: F, t31913: F, t31959: F, t32006: F, t32014: F, t32015: F, t33751: F, t385: F, t4742: F, t4772: F, t4872: F, t4946: F, t5015: F, t8501: F, t8507: F) -> F {
    let t126786 = -F::cast_from(0.17135921299530705785e1_f64) * t31903 * t31892 * t8507 * t4772 + F::cast_from(0.17135921299530705785e1_f64) * t126749 * t31899 + F::cast_from(0.24791552806034007214e-3_f64) * t120263 * t1042 * t4872 * t126600 + F::cast_from(0.56468933516960933998e-3_f64) * t3046 * t1032 * t8501 * t33751 + F::cast_from(0.56468933516960933998e-3_f64) * t31913 * t247 * t3116 * t385 * t4742 + F::cast_from(0.56468933516960933998e-3_f64) * t32014 * t32015 * t126765 * t4946 - F::cast_from(0.5578099381357651623e-3_f64) * t126770 * t32006 + F::cast_from(0.5578099381357651623e-3_f64) * t126774 * t1028 + F::cast_from(0.24791552806034007213e-3_f64) * t126779 - F::cast_from(0.12395776403017003607e-3_f64) * t120479 - F::cast_from(0.17135921299530705785e1_f64) * t31891 * t31959 * t8507 * t5015;
    t126786
}
