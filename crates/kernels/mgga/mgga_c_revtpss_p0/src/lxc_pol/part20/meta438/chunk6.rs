//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1657/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1657<F: Float>(t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F) -> (F, F) {
    let t45231 = -F::cast_from(0.25367901234567901233e-1_f64) * t43858 - F::cast_from(0.50735802469135802467e-1_f64) * t43862 - F::cast_from(0.13698666666666666667e0_f64) * t43830 - F::cast_from(0.3044148148148148148e-1_f64) * t43865 + F::cast_from(0.4566222222222222222e-1_f64) * t43832 + F::cast_from(0.11415555555555555555e0_f64) * t43837 - F::cast_from(0.34246666666666666665e-1_f64) * t43871 - F::cast_from(0.4566222222222222222e-1_f64) * t43841 + F::new(0.61644e0) * t43845 + F::new(0.10274e0) * t43877 + F::cast_from(0.13698666666666666667e0_f64) * t43849;
    let t45232 = F::cast_from(0.17757530864197530864e0_f64) * t43813;
    (t45231, t45232)
}
