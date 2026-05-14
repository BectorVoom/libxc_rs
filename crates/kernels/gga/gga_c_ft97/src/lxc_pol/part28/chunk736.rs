//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 736/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk736<F: Float>(t33039: F, t379: F, t2221: F, t609: F, t7407: F, t2179: F, t144: F, t1901: F, t33008: F, t33009: F, t33012: F, t33016: F, t33017: F, t33020: F, t33024: F, t33028: F, t33031: F, t33036: F, t446: F) -> (F, F, F, F, F, F) {
    let t33040 = t33039 * t379;
    let t33041 = t2221 * t33040;
    let t33044 = t7407 * t609;
    let t33045 = t2179 * t33044;
    let t33046 = t144 * t33045;
    let t33049 = t33008 - t446 * t33009 / 3.0 - 2.0 / 3.0 * t446 * t33012 + t33016 - 2.0 / 3.0 * t446 * t33017 - t446 * t33020 / 3.0 + 2.0 / 3.0 * t446 * t33024 - t446 * t33028 / 3.0 + 2.0 / 9.0 * t1901 * t33031 + t1901 * t33036 / 9.0 + t1901 * t33041 / 9.0 + 2.0 / 3.0 * t446 * t33046;
    (t33040, t33041, t33044, t33045, t33046, t33049)
}
