//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 838/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk838<F: Float>(t11593: F, t12676: F, t16979: F, t16983: F, t16986: F, t16990: F, t16993: F, t16998: F, t17003: F, t17008: F, t17013: F, t17018: F, t17023: F, t17027: F, t17032: F, t17035: F, t1901: F, t446: F) -> F {
    let t17038 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16979 - t446 * t16983 / F::cast_from(9.0_f64) + t12676 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16986 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t16990 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16993 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16998 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t17003 + t1901 * t17008 / F::cast_from(9.0_f64) + t1901 * t17013 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17018 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17023 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t17027 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t17032 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17035;
    t17038
}
