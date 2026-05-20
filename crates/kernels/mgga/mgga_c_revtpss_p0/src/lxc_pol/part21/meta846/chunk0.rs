//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3165/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3165<F: Float>(t43828: F, t43830: F, t43832: F, t43911: F, t56174: F, t56176: F, t56181: F, t58055: F, t58057: F, t58060: F, t58063: F, t58107: F) -> F {
    let t58386 = F::cast_from(0.247573125e0_f64) * t58055 + F::new(0.82524375e-1) * t58057 - F::cast_from(0.485484375e1_f64) * t58060 + F::cast_from(0.6189328125e-1_f64) * t58063 + F::new(0.16504875e0) * t58107 - F::new(0.33114e0) * t43828 - F::cast_from(0.60385000000000000002e0_f64) * t43830 + F::cast_from(0.20128333333333333334e0_f64) * t43832 - F::cast_from(0.91983333333333333335e-1_f64) * t43911 - F::cast_from(0.89459259259259259259e0_f64) * t56174 - F::cast_from(0.26837777777777777778e0_f64) * t56176 + F::cast_from(0.40256666666666666666e1_f64) * t56181;
    t58386
}
