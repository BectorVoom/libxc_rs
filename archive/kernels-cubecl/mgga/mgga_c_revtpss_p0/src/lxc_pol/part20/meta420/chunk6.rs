//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1571/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1571<F: Float>(t43808: F, t43810: F, t43814: F, t43817: F, t43823: F, t43826: F, t43828: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43854: F) -> F {
    let t43856 = -F::cast_from(0.247573125e0_f64) * t43808 + F::cast_from(0.3300975e0_f64) * t43810 + t43814 + t43817 - F::cast_from(0.485484375e1_f64) * t43823 - F::cast_from(0.3883875e1_f64) * t43826 - F::cast_from(0.132456e1_f64) * t43828 - F::cast_from(0.24154e1_f64) * t43830 + F::cast_from(0.80513333333333333333e0_f64) * t43832 + F::cast_from(0.20128333333333333334e1_f64) * t43837 - F::cast_from(0.80513333333333333332e0_f64) * t43841 + F::cast_from(0.108693e2_f64) * t43845 + F::cast_from(0.24154e1_f64) * t43849 - F::cast_from(0.72462e1_f64) * t43854;
    t43856
}
