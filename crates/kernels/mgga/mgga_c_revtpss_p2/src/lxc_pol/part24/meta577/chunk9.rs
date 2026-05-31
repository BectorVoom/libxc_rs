//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1778/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1778<F: Float>(t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t45232: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> (F, F) {
    let t90836 = F::cast_from(0.11415555555555555555e0_f64) * t89824 - F::cast_from(0.41095999999999999998e0_f64) * t89828 - F::cast_from(0.50735802469135802467e-1_f64) * t89832 + F::cast_from(0.4566222222222222222e-1_f64) * t81156 - F::cast_from(0.13698666666666666667e0_f64) * t81158 + F::cast_from(0.45662222222222222221e-1_f64) * t68255 - F::cast_from(0.34246666666666666665e-1_f64) * t89839 - F::cast_from(0.4566222222222222222e-1_f64) * t89843 + F::cast_from(0.61644e0_f64) * t89847 + F::cast_from(0.10274e0_f64) * t89851 + F::cast_from(0.13698666666666666667e0_f64) * t89855;
    let t90848 = -F::cast_from(0.25367901234567901233e-1_f64) * t81230 + F::cast_from(0.9132444444444444444e-1_f64) * t81232 - F::cast_from(0.3044148148148148148e-1_f64) * t68257 - F::cast_from(0.13698666666666666667e0_f64) * t81234 - F::cast_from(0.22831111111111111111e-1_f64) * t81236 + F::cast_from(0.2283111111111111111e0_f64) * t89865 - F::cast_from(0.41095999999999999999e0_f64) * t89869 + F::cast_from(0.41096e0_f64) * t89873 + F::cast_from(0.17123333333333333333e-1_f64) * t89877 - F::cast_from(0.71030123456790123454e-1_f64) * t56236 + t45232 + F::cast_from(0.9132444444444444444e-1_f64) * t68399;
    (t90836, t90848)
}
