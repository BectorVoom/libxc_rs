//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1116/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1116<F: Float>(t14965: F, t11739: F, t11743: F, t11747: F, t11756: F, t11770: F, t11772: F, t19977: F, t19978: F, t19979: F, t19980: F, t19981: F, t19982: F, t19984: F, t19986: F, t19988: F, t19989: F, t19990: F) -> (F, F) {
    let t19991 = F::new(48.0) * t14965;
    let t19992 = -t19977 - t19978 + t11739 - t11743 + t19979 + t11747 - t19980 + t19981 - t11756 + t19982 - t19984 + t19986 - t19988 - t19989 - t19990 - t19991 + t11770 - t11772;
    (t19991, t19992)
}
