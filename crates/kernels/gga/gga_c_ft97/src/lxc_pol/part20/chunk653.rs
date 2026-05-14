//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 653/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk653<F: Float>(t13702: F, t14196: F, t265: F, t9802: F, t13706: F, t2409: F, t3869: F, t2606: F, t10000: F, t10012: F, t14156: F, t14160: F, t14164: F, t14168: F, t14172: F, t14177: F, t14184: F, t14189: F, t14193: F, t1901: F, t9997: F) -> (F, F, F) {
    let t14197 = t14196 * t13702;
    let t14200 = t9802 * t265;
    let t14201 = t14200 * t13706;
    let t14205 = t3869 * t2409;
    let t14206 = t2606 * t14205;
    let t14209 = 2.0 / 9.0 * t1901 * t14156 + 2.0 / 9.0 * t1901 * t14160 - 4.0 / 9.0 * t1901 * t14164 - 2.0 / 9.0 * t1901 * t14168 - 2.0 / 9.0 * t1901 * t14172 - 4.0 / 9.0 * t1901 * t14177 - t9997 / 9.0 + 8.0 / 27.0 * t10000 - 4.0 / 9.0 * t1901 * t14184 + 4.0 / 27.0 * t1901 * t14189 - 2.0 / 9.0 * t1901 * t14193 - 4.0 / 9.0 * t1901 * t14197 + 4.0 / 27.0 * t1901 * t14201 - 2.0 / 27.0 * t10012 - 2.0 / 9.0 * t1901 * t14206;
    (t14200, t14205, t14209)
}
