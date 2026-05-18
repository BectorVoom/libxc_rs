//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1024/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1024<F: Float>(t19249: F, t19252: F, t19255: F, t19258: F, t19261: F, t19265: F, t19269: F, t19754: F, t19757: F, t19761: F, t19838: F, t19278: F) -> (F, F) {
    let t19839 = F::new(2.0) / F::new(3.0) * t19249;
    let t19849 = t19838 - t19839 - t19754 / F::new(4.0) - t19757 / F::new(2.0) + F::new(3.0) / F::new(8.0) * t19761 + F::new(2.0) / F::new(3.0) * t19252 - F::new(2.0) / F::new(9.0) * t19255 - F::new(10.0) / F::new(27.0) * t19258 + F::new(8.0) / F::new(9.0) * t19261 + F::new(2.0) / F::new(3.0) * t19265 - F::new(4.0) / F::new(3.0) * t19269;
    let t19852 = F::new(2.0) / F::new(9.0) * t19278;
    (t19849, t19852)
}
