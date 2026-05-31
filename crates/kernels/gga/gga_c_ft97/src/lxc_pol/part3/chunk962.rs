//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 962/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk962<F: Float>(t2253: F, t5442: F, t10838: F, t10921: F, t14421: F, t14423: F, t14429: F, t14431: F, t14445: F, t14448: F, t14478: F, t14480: F, t14482: F, t18820: F, t18823: F, t18825: F, t18854: F, t18859: F, t18864: F, t18867: F, t18871: F, t2265: F, t631: F) -> F {
    let t18874 = t2253 * t5442;
    let t18876 = t14421 + t14423 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14429 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t14431 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t14445 - t14448 + t14478 + t14480 - t14482 + t10838 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t10921 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t18820 - t18823 / F::cast_from(3.0_f64) + t18825 + t631 * t18854 / F::cast_from(2.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2265 * t18859 + F::cast_from(2.0_f64) * t2265 * t18864 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t18867 - F::cast_from(3.0_f64) * t631 * t18871 - t18874 / F::cast_from(27.0_f64);
    t18876
}
