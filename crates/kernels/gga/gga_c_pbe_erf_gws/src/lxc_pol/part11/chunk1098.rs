//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1098/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1098<F: Float>(t30824: F, t30839: F, t16705: F, t31785: F, t31805: F, t40163: F, t40213: F, t40251: F, t47407: F, t47412: F, t47416: F, t47470: F, t47473: F) -> (F, F, F) {
    let t47586 = F::new(32.0) / F::new(45.0) * t30824;
    let t47587 = F::new(8.0) / F::new(45.0) * t30839;
    let t47598 = F::new(0.50377777777777777778e-2) * t31785 - F::new(0.5037777777777777778e-2) * t40213 + F::new(0.15113333333333333333e-1) * t40163 - t16705 + F::new(0.33585185185185185186e-2) * t31805 - F::new(0.27987654320987654323e-2) * t40251 + F::new(0.45340000000000000001e-1) * t47407 - F::new(0.45340000000000000002e-1) * t47470 + F::new(0.37783333333333333335e-2) * t47412 + F::new(0.5037777777777777778e-2) * t47473 - F::new(0.4534e-1) * t47416;
    (t47586, t47587, t47598)
}
