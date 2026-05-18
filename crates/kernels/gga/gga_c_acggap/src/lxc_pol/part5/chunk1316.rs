//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1316/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1316<F: Float>(t11739: F, t11743: F, t11747: F, t14974: F, t1680: F, t19973: F, t19974: F, t19975: F, t19977: F, t19978: F, t19979: F, t19980: F, t694: F) -> F {
    let t24564 = -F::new(6.0) * t14974 * t1680 * t694 + t11739 - t11743 + t11747 + t19973 + t19974 + t19975 - t19977 - t19978 + t19979 - t19980;
    t24564
}
