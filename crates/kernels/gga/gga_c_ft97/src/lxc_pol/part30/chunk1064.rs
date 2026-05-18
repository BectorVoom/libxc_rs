//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1064/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1064<F: Float>(t141340: F, t141651: F, t141652: F, t150953: F, t150958: F, t150962: F, t150966: F, t150971: F, t150974: F, t150977: F, t150980: F, t150983: F, t150985: F, t150988: F, t150992: F, t150996: F) -> F {
    let t151327 = -F::new(2.0) / F::new(9.0) * t141340 + t150953 - F::new(5.0) / F::new(4.0) * t150958 - t150962 / F::new(6.0) + F::new(4.0) / F::new(9.0) * t150966 + t150971 / F::new(9.0) + F::new(4.0) / F::new(9.0) * t150974 - F::new(4.0) / F::new(27.0) * t150977 + F::new(2.0) / F::new(3.0) * t150980 - F::new(4.0) / F::new(9.0) * t150983 + t141651 - t141652 + F::new(2.0) / F::new(27.0) * t150985 + t150988 / F::new(18.0) + F::new(4.0) / F::new(3.0) * t150992 + F::new(2.0) / F::new(3.0) * t150996;
    t151327
}
