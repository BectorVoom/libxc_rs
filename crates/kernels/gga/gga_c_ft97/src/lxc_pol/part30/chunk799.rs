//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 799/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk799<F: Float>(t33866: F, t33959: F, t33976: F, t33871: F, t33876: F, t33956: F, t33964: F, t33969: F, t33973: F, t33981: F, t33986: F, t33990: F) -> (F, F, F, F) {
    let t34042 = t33866 / F::cast_from(6.0_f64);
    let t34045 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33959;
    let t34049 = t33976 / F::cast_from(3.0_f64);
    let t34052 = t34042 + t33871 / F::cast_from(6.0_f64) + t33876 - t33956 / F::cast_from(2.0_f64) - t34045 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33964 - F::cast_from(6.0_f64) * t33969 + F::cast_from(4.0_f64) * t33973 + t34049 + t33981 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t33986 - t33990;
    (t34042, t34045, t34049, t34052)
}
