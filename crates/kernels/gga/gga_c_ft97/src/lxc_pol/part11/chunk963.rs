//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 963/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk963<F: Float>(t41950: F, t41947: F, t41953: F, t41957: F, t41960: F, t41964: F, t41969: F, t41973: F, t41978: F, t41981: F, t42053: F, t42057: F, t42233: F, t42236: F, t42240: F, t42724: F, t42740: F, t42757: F, t762: F) -> (F,) {
    let t42759 = 280.0 / 81.0 * t41950;
    let t42772 = 4.0 / 3.0 * t41947 + t42759 - 15.0 / 16.0 * t42053 - 3.0 / 4.0 * t42057 + t42233 / 2.0 - t42236 + 9.0 / 4.0 * t42240 - 8.0 / 9.0 * t41953 - 16.0 / 27.0 * t41957 - 16.0 / 9.0 * t41960 + 40.0 / 81.0 * t41964 + 40.0 / 9.0 * t41969 - t41973 / 3.0 - 36.0 * t41978 + 112.0 / 81.0 * t41981;
    let t42775 = t762 * (t42724 + t42740 + t42757 + t42772);
    (t42775,)
}
