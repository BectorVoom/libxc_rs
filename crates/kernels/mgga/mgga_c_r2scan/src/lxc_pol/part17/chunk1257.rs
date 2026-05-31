//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1257/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1257<F: Float>(t1018: F, t1125: F, t12267: F, t12849: F, t12851: F, t12854: F, t12856: F, t2405: F, t2406: F, t2951: F, t2953: F, t330: F, t3517: F, t3740: F, t3742: F, t44609: F, t44661: F, t837: F, t838: F, t9698: F) -> F {
    let t44684 = (t44609 + t44661) * t330 + t12849 * t837 * t330 + F::cast_from(2.0_f64) * t12267 * t1018 * t330 + F::cast_from(2.0_f64) * t3740 * t2405 * t330 + F::cast_from(2.0_f64) * t12851 * t838 + t3517 * t2951 * t330 + t1125 * t9698 * t330 + t12854 * t838 + t3517 * t2953 * t330 + F::cast_from(2.0_f64) * t3742 * t2406 + t12856 * t838;
    t44684
}
