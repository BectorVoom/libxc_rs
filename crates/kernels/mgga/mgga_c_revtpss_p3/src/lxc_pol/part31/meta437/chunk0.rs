//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1561/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1561<F: Float>(t1045: F, t19477: F, t373: F, t1042: F, t18909: F, t4919: F, t1011: F, t1041: F, t11732: F, t11737: F, t15656: F, t15732: F, t15736: F, t15744: F, t15750: F, t15754: F, t1665: F, t4854: F, t4858: F) -> (F, F) {
    let t19799 = t373 * t19477 * t1045;
    let t19800 = t1042 * t19799;
    let t19809 = t4919 * t18909;
    let t19813 = F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t19800 - F::cast_from(0.95275595817932748827e-4_f64) * t15732 - t15736 - F::cast_from(0.42874018118069736972e-3_f64) * t15656 * t1665 - F::cast_from(0.42874018118069736972e-3_f64) * t4858 * t4854 + t15744 + F::cast_from(0.95275595817932748827e-4_f64) * t15750 - t1011 * t19809 / F::new(36.0) - t15754 + t11732 / F::new(162.0) + t11737;
    (t19800, t19813)
}
