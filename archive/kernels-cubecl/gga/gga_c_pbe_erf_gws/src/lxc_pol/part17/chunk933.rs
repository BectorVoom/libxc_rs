//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 933/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk933<F: Float>(t2900: F, t513: F, t1576: F, t981: F, t1578: F, t985: F, t2919: F, t520: F, t1590: F, t5753: F, t5755: F, t5776: F, t5863: F, t5864: F, t5866: F, t5874: F, t8117: F, t8137: F, t8142: F, t8145: F, t8171: F, t8174: F) -> (F, F, F, F, F, F) {
    let t8206 = t2900 * t513;
    let t8209 = t981 * t1576;
    let t8218 = t985 * t1578;
    let t8221 = t2919 * t520;
    let t8224 = t985 * t1590;
    let t8230 = t5753 - t5755 - t5863 + t8117 - t5776 - t8137 + t8142 + t8145 + t8171 + t8174 - F::cast_from(0.15326711111111111111e1_f64) * t5864 - F::cast_from(0.1724255e1_f64) * t5866 + F::cast_from(0.57475166666666666666e0_f64) * t5874;
    (t8206, t8209, t8218, t8221, t8224, t8230)
}
