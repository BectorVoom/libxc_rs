//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 899/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk899<F: Float>(t265: F, t266: F, t837: F, t245: F, t5420: F, t712: F, t5427: F, t723: F, t1903: F, t1924: F, t1917: F, t703: F) -> (F, F, F, F, F) {
    let t18280 = F::new(56.0) / F::new(1215.0) * t265 * t266 * t837;
    let t18309 = F::new(0.2e-20) * t712 * t245 * t5420;
    let t18311 = F::new(8.0) / F::new(9.0) * t5427 * t723;
    let t18315 = F::new(4.0) / F::new(9.0) * t1924 * t1903;
    let t18318 = F::cast_from(0.5402469135802469136e-1_f64) * t712 * t703 * t1917;
    (t18280, t18309, t18311, t18315, t18318)
}
