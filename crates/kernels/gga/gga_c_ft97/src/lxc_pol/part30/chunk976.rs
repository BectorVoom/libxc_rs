//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 976/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk976<F: Float>(t143365: F, t1882: F, t33980: F, t33953: F, t668: F, t25409: F, t7581: F, t143263: F, t143273: F, t143332: F, t143335: F, t34281: F, t6210: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t143366 = F::new(2.0) / F::new(27.0) * t143365;
    let t143371 = t1882 * t33980;
    let t143373 = t33953 * t668;
    let t143432 = t7581 * t25409;
    let t143497 = F::new(8.0) / F::new(9.0) * t143263;
    let t143500 = F::new(10.0) / F::new(9.0) * t143273;
    let t143518 = F::new(4.0) / F::new(9.0) * t143332;
    let t143519 = F::new(4.0) / F::new(9.0) * t143335;
    let t143528 = F::new(2.0) / F::new(9.0) * t143365;
    let t143538 = t6210 * t34281;
    (t143366, t143371, t143373, t143432, t143497, t143500, t143518, t143519, t143528, t143538)
}
