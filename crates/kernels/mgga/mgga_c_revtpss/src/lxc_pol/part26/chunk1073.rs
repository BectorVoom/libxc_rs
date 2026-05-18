//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1073/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1073<F: Float>(t25973: F, t25979: F, t25984: F, t25988: F, t25990: F, t25992: F, t25994: F, t25998: F, t26310: F, t26312: F, t26332: F) -> F {
    let t26333 = -t26310 - F::new(0.4065600224742826258e-3) * t25973 + t26312 + F::new(0.32012600194825403606e-1) * t25979 + F::new(0.17149607247227894789e-2) * t25984 + F::new(0.57165357490759649296e-4) * t25988 - F::new(0.34299214494455789578e-2) * t25990 + F::new(0.17149607247227894789e-1) * t25992 - F::new(0.85748036236139473944e-3) * t25994 - F::new(0.10164000561857065645e-3) * t25998 + t26332;
    t26333
}
