//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 971/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk971<F: Float>(t30824: F, t30839: F, t16705: F, t31785: F, t31805: F, t40163: F, t40213: F, t40251: F, t47407: F, t47412: F, t47416: F, t47470: F, t47473: F, t24088: F, t31643: F, t40255: F, t40262: F, t40264: F, t47420: F, t47423: F, t47426: F, t47430: F, t47476: F, t47479: F, t47482: F) -> (F, F, F, F) {
    let t47586 = 32.0 / 45.0 * t30824;
    let t47587 = 8.0 / 45.0 * t30839;
    let t47598 = 0.50377777777777777778e-2 * t31785 - 0.5037777777777777778e-2 * t40213 + 0.15113333333333333333e-1 * t40163 - t16705 + 0.33585185185185185186e-2 * t31805 - 0.27987654320987654323e-2 * t40251 + 0.45340000000000000001e-1 * t47407 - 0.45340000000000000002e-1 * t47470 + 0.37783333333333333335e-2 * t47412 + 0.5037777777777777778e-2 * t47473 - 0.4534e-1 * t47416;
    let t47611 = 0.6801e-1 * t47476 - 0.11335e-1 * t47420 - 0.15113333333333333333e-1 * t47479 - 0.25188888888888888889e-2 * t40255 - 0.2518888888888888889e-1 * t47423 + 0.12594444444444444445e-1 * t47482 - 0.78365432098765432099e-2 * t24088 + 0.10075555555555555556e-1 * t40262 - 0.15113333333333333333e-1 * t40264 - 0.10075555555555555556e-1 * t31643 + 0.55975308641975308645e-2 * t47426 + 0.18891666666666666667e-2 * t47430;
    (t47586, t47587, t47598, t47611)
}
