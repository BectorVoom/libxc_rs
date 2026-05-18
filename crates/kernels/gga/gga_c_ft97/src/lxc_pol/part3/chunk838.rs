//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 838/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk838<F: Float>(t11593: F, t12676: F, t16979: F, t16983: F, t16986: F, t16990: F, t16993: F, t16998: F, t17003: F, t17008: F, t17013: F, t17018: F, t17023: F, t17027: F, t17032: F, t17035: F, t1901: F, t446: F) -> F {
    let t17038 = F::new(2.0) / F::new(3.0) * t446 * t16979 - t446 * t16983 / F::new(9.0) + t12676 - F::new(2.0) / F::new(27.0) * t16986 + F::new(4.0) / F::new(9.0) * t11593 * t16990 + F::new(2.0) / F::new(9.0) * t1901 * t16993 + F::new(2.0) / F::new(9.0) * t1901 * t16998 + F::new(4.0) / F::new(9.0) * t11593 * t17003 + t1901 * t17008 / F::new(9.0) + t1901 * t17013 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t17018 - F::new(2.0) / F::new(9.0) * t1901 * t17023 - F::new(4.0) / F::new(3.0) * t1901 * t17027 - F::new(4.0) / F::new(3.0) * t1901 * t17032 + F::new(2.0) / F::new(9.0) * t1901 * t17035;
    t17038
}
