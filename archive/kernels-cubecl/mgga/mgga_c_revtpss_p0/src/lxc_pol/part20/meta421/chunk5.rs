//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1577/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577<F: Float>(t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F) -> (F, F) {
    let t43880 = -F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t43858 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t43862 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43830 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t43865 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43832 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t43837 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43871 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43841 + F::cast_from(12.0_f64) * t43845 + F::cast_from(2.0_f64) * t43877 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43849;
    let t43881 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t43813;
    (t43880, t43881)
}
