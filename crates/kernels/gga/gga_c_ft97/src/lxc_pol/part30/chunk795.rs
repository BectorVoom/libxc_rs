//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 795/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk795<F: Float>(t33983: F, t824: F, t193: F, t89: F, t33953: F, t799: F, t27: F, t33867: F, t33871: F, t33876: F, t33956: F, t33960: F, t33964: F, t33969: F, t33973: F, t33977: F, t33981: F) -> (F, F, F, F, F) {
    let t33984 = t33983 * t824;
    let t33985 = t193 * t33984;
    let t33986 = t89 * t33985;
    let t33988 = t799 * t33953;
    let t33990 = t89 * t27 * t33988;
    let t33992 = t33867 + t33871 / F::new(18.0) + t33876 / F::new(3.0) - t33956 / F::new(6.0) - t33960 - F::new(2.0) / F::new(9.0) * t33964 - F::new(2.0) * t33969 + F::new(4.0) / F::new(3.0) * t33973 + t33977 + t33981 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t33986 - t33990 / F::new(3.0);
    (t33984, t33986, t33988, t33990, t33992)
}
