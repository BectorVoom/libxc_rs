//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2514/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2514<F: Float>(t50977: F, t40672: F, t828: F, t14819: F, t40517: F, t14741: F, t2710: F, t2713: F, t10744: F, t14861: F, t808: F, t40791: F, t4442: F) -> (F, F, F, F, F, F) {
    let t50978 = F::cast_from(0.30492001685571196935e-4_f64) * t50977;
    let t51014 = t40672 * t828;
    let t51042 = t40517 * t14819;
    let t51055 = t2710 * t2713 * t14741;
    let t51058 = t10744 * t808 * t14861;
    let t51059 = F::cast_from(0.76230004213927992336e-5_f64) * t51058;
    let t51060 = t40791 * t4442;
    (t50978, t51014, t51042, t51055, t51059, t51060)
}
