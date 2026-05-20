//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3475/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475<F: Float>(t2986: F, t63902: F, t973: F, t981: F, t19468: F, t3022: F, t19021: F, t974: F, t2988: F, t41235: F, t41238: F, t6189: F) -> (F, F, F, F) {
    let t65402 = F::cast_from(0.23392894490538584828e1_f64) * t981 * t2986 * t63902 * t973;
    let t65404 = F::cast_from(0.34631718211362927518e2_f64) * t3022 * t19468;
    let t65408 = F::cast_from(0.23392894490538584828e1_f64) * t981 * t2986 * t19021 * t974;
    let t65413 = F::cast_from(0.91082604192152556044e5_f64) * t981 * t41235 * t6189 * t41238 * t2988;
    (t65402, t65404, t65408, t65413)
}
