//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 705/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk705<F: Float>(t27063: F, t27112: F, t27141: F, t27189: F, t605: F, t144: F, t11593: F, t1901: F, t26978: F, t26982: F, t26985: F, t26988: F, t26992: F, t26996: F, t27001: F, t27004: F, t27008: F, t27012: F, t27017: F, t27022: F, t27025: F, t446: F) -> (F, F) {
    let t27191 = t27063 + t27112 + t27141 + t27189;
    let t27192 = t605 * t27191;
    let t27193 = t144 * t27192;
    let t27196 = -t446 * t26978 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1901 * t26982 + t1901 * t26985 / F::new(9.0) + t1901 * t26988 / F::new(9.0) + t1901 * t26992 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t11593 * t26996 - F::new(2.0) * t1901 * t27001 + t27004 / F::new(9.0) + t1901 * t27008 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t1901 * t27012 - F::new(2.0) / F::new(3.0) * t1901 * t27017 + t1901 * t27022 / F::new(9.0) - t27025 / F::new(27.0) - t446 * t27193 / F::new(3.0);
    (t27191, t27196)
}
