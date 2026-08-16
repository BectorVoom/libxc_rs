//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1194/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1194<F: Float>(t80885: F, t80899: F, t80876: F, t80878: F, t80889: F, t80897: F, t80904: F, t80906: F, t80908: F, t80911: F, t80915: F, t80918: F, t80920: F, t80922: F, t80925: F, t80928: F, t80931: F, t80934: F, t80937: F, t80940: F) -> F {
    let t84533 = F::cast_from(0.67287926823567318088e-4_f64) * t80885;
    let t84536 = F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t80899;
    let t84551 = -t80876 / F::cast_from(64.0_f64) - t80878 / F::cast_from(192.0_f64) - t84533 - F::cast_from(0.35608770875031824732e0_f64) * t80889 - F::cast_from(0.13565246047631171326e0_f64) * t80897 - t84536 - t80904 / F::cast_from(128.0_f64) + t80906 / F::cast_from(128.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t80908 - t80911 / F::cast_from(256.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t80915 - F::cast_from(0.12111826828242117256e-2_f64) * t80918 + F::cast_from(0.84782787797694820791e-2_f64) * t80920 + F::cast_from(0.84782787797694820791e-2_f64) * t80922 - F::cast_from(0.40372756094140390853e-3_f64) * t80925 - F::cast_from(0.40372756094140390853e-3_f64) * t80928 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t80931 + F::cast_from(0.50869672678616892474e-1_f64) * t80934 + F::cast_from(0.24223653656484234512e-2_f64) * t80937 - F::cast_from(0.67826230238155856633e-1_f64) * t80940;
    t84551
}
