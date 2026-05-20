//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1740/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1740<F: Float>(t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t44865: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> (F, F) {
    let t89947 = F::cast_from(0.98777777777777777779e-1_f64) * t89824 - F::cast_from(0.35560000000000000001e0_f64) * t89828 - F::cast_from(0.43901234567901234568e-1_f64) * t89832 + F::cast_from(0.39511111111111111112e-1_f64) * t81156 - F::cast_from(0.11853333333333333334e0_f64) * t81158 + F::cast_from(0.39511111111111111112e-1_f64) * t68255 - F::cast_from(0.29633333333333333334e-1_f64) * t89839 - F::cast_from(0.39511111111111111112e-1_f64) * t89843 + F::cast_from(0.53340000000000000002e0_f64) * t89847 + F::cast_from(0.88900000000000000002e-1_f64) * t89851 + F::cast_from(0.11853333333333333334e0_f64) * t89855;
    let t89959 = -F::cast_from(0.21950617283950617284e-1_f64) * t81230 + F::cast_from(0.79022222222222222224e-1_f64) * t81232 - F::cast_from(0.26340740740740740742e-1_f64) * t68257 - F::cast_from(0.11853333333333333334e0_f64) * t81234 - F::cast_from(0.19755555555555555556e-1_f64) * t81236 + F::cast_from(0.19755555555555555556e0_f64) * t89865 - F::cast_from(0.35560000000000000001e0_f64) * t89869 + F::cast_from(0.35560000000000000001e0_f64) * t89873 + F::cast_from(0.14816666666666666667e-1_f64) * t89877 - F::cast_from(0.61461728395061728396e-1_f64) * t56236 + t44865 + F::cast_from(0.79022222222222222224e-1_f64) * t68399;
    (t89947, t89959)
}
