//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 999/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk999<F: Float>(t1286: F, t34619: F, t376: F, t100089: F, t101983: F, t1308: F, t136016: F, t136018: F, t136037: F, t136041: F, t136077: F, t1564: F, t22873: F, t22935: F, t25861: F, t26493: F, t28: F, t32406: F, t34787: F, t5501: F, t5507: F, t6414: F, t925: F) -> F {
    let t144350 = t1286 * t376 * t34619;
    let t144372 = t136016 / F::cast_from(54.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6414 * t32406 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1286 * t28 * t5507 * t101983 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1286 * t28 * t22873 * t25861 + t144350 / F::cast_from(9.0_f64) + t1286 * t28 * t1308 * t26493 / F::cast_from(3.0_f64) - t5501 * t1564 * t136077 * t925 / F::cast_from(9.0_f64) - t22935 * t34787 / F::cast_from(9.0_f64) - t5501 * t1564 * t136018 * t925 / F::cast_from(9.0_f64) - t136037 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t136041 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1286 * t28 * t5507 * t100089;
    t144372
}
