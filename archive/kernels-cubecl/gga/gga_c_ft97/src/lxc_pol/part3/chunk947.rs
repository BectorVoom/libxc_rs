//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 947/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk947<F: Float>(t10000: F, t14212: F, t14223: F, t14224: F, t14232: F, t14233: F, t18602: F, t18606: F, t18610: F, t18614: F, t18619: F, t18624: F, t18628: F, t18633: F, t18636: F, t446: F) -> F {
    let t18639 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10000 - t446 * t18602 / F::cast_from(9.0_f64) - t446 * t18606 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t18610 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t18614 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18619 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t18624 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18628 + t14212 - t14223 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t14224 + t14232 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t14233 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18633 - t446 * t18636 / F::cast_from(9.0_f64);
    t18639
}
