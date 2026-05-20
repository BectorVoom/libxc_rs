//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1635/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1635<F: Float>(t12625: F, t458: F, t456: F, t225: F, t480: F, t3568: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F) -> (F, F, F, F, F) {
    let t44841 = F::new(1.0) / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    let t44844 = t44843 * t480;
    let t44845 = t3568 * t3568;
    let t44864 = -F::cast_from(0.21950617283950617284e-1_f64) * t43858 - F::cast_from(0.43901234567901234568e-1_f64) * t43862 - F::cast_from(0.11853333333333333334e0_f64) * t43830 - F::cast_from(0.26340740740740740742e-1_f64) * t43865 + F::cast_from(0.39511111111111111112e-1_f64) * t43832 + F::cast_from(0.98777777777777777779e-1_f64) * t43837 - F::cast_from(0.29633333333333333334e-1_f64) * t43871 - F::cast_from(0.39511111111111111112e-1_f64) * t43841 + F::cast_from(0.53340000000000000002e0_f64) * t43845 + F::cast_from(0.88900000000000000002e-1_f64) * t43877 + F::cast_from(0.11853333333333333334e0_f64) * t43849;
    (t44842, t44843, t44844, t44845, t44864)
}
