//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 933/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk933<F: Float>(t25970: F, t25976: F, t26013: F, t26015: F, t27933: F, t27937: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F, t27951: F, t27953: F, t27955: F, t27957: F) -> F {
    let t27959 = t27933 / F::new(16.0) - t25970 + t25976 + F::new(0.57165357490759649296e-4) * t26015 + F::new(0.57165357490759649296e-4) * t27937 + t26013 + F::new(0.85748036236139473944e-3) * t27941 + F::new(0.17149607247227894789e-2) * t27943 - F::new(0.42874018118069736972e-3) * t27945 + F::new(0.17149607247227894789e-2) * t27947 - F::new(0.17149607247227894789e-2) * t27949 - F::new(0.42874018118069736972e-3) * t27951 - F::new(0.25410001404642664113e-4) * t27953 + F::new(7.0) / F::new(144.0) * t27955 - t27957 / F::new(48.0);
    t27959
}
