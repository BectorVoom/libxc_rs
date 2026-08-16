//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1060/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1060(t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43750: f64, t43752: f64, t43754: f64, t43757: f64, t43759: f64, t43761: f64, t43766: f64, t43774: f64, t43775: f64, t47357: f64, t47360: f64, t47362: f64, t47364: f64, t47366: f64, t47368: f64, t47371: f64) -> f64 {
    let t51134 = t43737 - t43740 - t43743 - t43746 - t43750 - t43752 - 0.23005755572352449806e2_f64 * t47357 + 0.55213813373645879536e2_f64 * t47360 + t43754 - 0.14300195980740170668e1_f64 * t47362 + t43757 + 0.71500979903700853338e0_f64 * t47364 + 0.71500979903700853338e0_f64 * t47366 + 0.71500979903700853338e0_f64 * t47368 + 0.95334639871601137787e0_f64 * t47371 - t43759 + t43761 + t43766 + t43774 + t43775;
    t51134
}
