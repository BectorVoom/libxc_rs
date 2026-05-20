//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1171/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1171<F: Float>(t1035: F, t1651: F, t1646: F, t1043: F, t1089: F, t1096: F, t120297: F, t120321: F, t120322: F, t120448: F, t120513: F, t120636: F, t120664: F, t120702: F, t126442: F, t1976: F, t1982: F, t27543: F, t3153: F, t31891: F, t31892: F, t31897: F, t31903: F, t31959: F, t32015: F, t33787: F, t33791: F, t33796: F, t33803: F, t33817: F, t4742: F, t4763: F, t4998: F, t7135: F, t7145: F, t7837: F, t8507: F, t988: F) -> F {
    let t126953 = t1035 * t1651;
    let t126965 = t1035 * t1646;
    let t126995 = F::cast_from(0.11423947533020470523e1_f64) * t31891 * t31892 * t1976 * t27543 + F::cast_from(0.17347256376410398924e1_f64) * t120664 * t126953 * t1043 * t1089 + F::cast_from(0.34694512752820797848e1_f64) * t120448 * t7145 * t126442 + F::cast_from(0.8673628188205199462e0_f64) * t120636 * t33787 * t3153 * t4998 - F::cast_from(0.17347256376410398924e1_f64) * t120513 * t126965 * t1043 * t1089 + F::cast_from(0.51407763898592117355e1_f64) * t31903 * t31959 * t33796 * t1096 - F::cast_from(0.51407763898592117355e1_f64) * t31897 * t31959 * t33803 * t988 + F::cast_from(0.56468933516960933998e-3_f64) * t120321 * t32015 * t120322 * t4763 - F::cast_from(0.5578099381357651623e-3_f64) * t120297 * t33817 + F::cast_from(0.17135921299530705785e1_f64) * t31897 * t31892 * t8507 * t4742 - F::cast_from(0.17347256376410398924e1_f64) * t1982 * t7135 * t7837 + F::cast_from(0.3427184259906141157e1_f64) * t120702 * t31892 * t33791 * t988;
    t126995
}
