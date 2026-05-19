//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1059/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1059<F: Float>(t43737: F, t43740: F, t43743: F, t43746: F, t43750: F, t43752: F, t43754: F, t43757: F, t43759: F, t43761: F, t43766: F, t43774: F, t43775: F, t47357: F, t47360: F, t47362: F, t47364: F, t47366: F, t47368: F, t47371: F) -> F {
    let t51134 = t43737 - t43740 - t43743 - t43746 - t43750 - t43752 - F::cast_from(0.23005755572352449806e2_f64) * t47357 + F::cast_from(0.55213813373645879536e2_f64) * t47360 + t43754 - F::cast_from(0.14300195980740170668e1_f64) * t47362 + t43757 + F::cast_from(0.71500979903700853338e0_f64) * t47364 + F::cast_from(0.71500979903700853338e0_f64) * t47366 + F::cast_from(0.71500979903700853338e0_f64) * t47368 + F::cast_from(0.95334639871601137787e0_f64) * t47371 - t43759 + t43761 + t43766 + t43774 + t43775;
    t51134
}
