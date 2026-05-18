//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1058/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1058<F: Float>(t1403: F, t35296: F, t681: F, t27968: F, t7437: F, t150036: F, t150040: F, t150044: F, t150047: F, t150051: F, t150054: F, t150058: F, t150062: F, t150066: F, t150069: F, t150073: F, t150077: F, t150079: F, t150084: F, t150088: F, t150092: F) -> (F, F, F) {
    let t151200 = t1403 * t681 * t35296;
    let t151212 = t7437 * t27968;
    let t151230 = F::new(2.0) / F::new(27.0) * t150036 - t150040 / F::new(9.0) + t150044 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t150047 - t150051 / F::new(3.0) - F::new(2.0) * t150054 + t150058 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t150062 + F::new(2.0) / F::new(9.0) * t150066 - F::new(2.0) / F::new(27.0) * t150069 + t150073 / F::new(18.0) + t150077 / F::new(18.0) - t150079 / F::new(54.0) + t150084 / F::new(18.0) - t150088 / F::new(9.0) + t150092 / F::new(2.0);
    (t151200, t151212, t151230)
}
