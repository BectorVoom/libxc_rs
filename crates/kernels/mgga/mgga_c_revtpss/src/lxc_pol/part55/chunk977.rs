//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 977/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk977<F: Float>(t27937: F, t27955: F, t26016: F, t26310: F, t26312: F, t26325: F, t27933: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F, t27951: F, t27953: F, t27957: F) -> F {
    let t28877 = F::new(0.11433071498151929859e-3) * t27937;
    let t28885 = F::new(7.0) / F::new(72.0) * t27955;
    let t28887 = t27933 / F::new(8.0) - t26310 + t26312 + t26016 + t28877 + t26325 + F::new(0.17149607247227894789e-2) * t27941 + F::new(0.34299214494455789578e-2) * t27943 - F::new(0.85748036236139473944e-3) * t27945 + F::new(0.34299214494455789578e-2) * t27947 - F::new(0.34299214494455789578e-2) * t27949 - F::new(0.85748036236139473944e-3) * t27951 - F::new(0.50820002809285328225e-4) * t27953 + t28885 - t27957 / F::new(24.0);
    t28887
}
