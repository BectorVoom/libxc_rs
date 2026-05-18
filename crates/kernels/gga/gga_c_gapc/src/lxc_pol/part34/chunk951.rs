//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 951/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk951<F: Float>(t10000: F, t10003: F, t10006: F, t10010: F, t10014: F, t10016: F, t10019: F, t10021: F, t9986: F, t9991: F, t9993: F, t9995: F, t9997: F) -> F {
    let t11020 = F::new(0.29517957899305555558e-5) * t9986 + F::new(0.84410248952307505288e-7) * t9991 - F::new(0.2748593934505475288e-5) * t9993 + F::new(0.2162369695472428426e-1) * t9995 + F::new(0.56273499301538336858e-7) * t9997 + F::new(0.56273499301538336858e-7) * t10000 - F::new(0.27801896084645508334e-2) * t10003 - F::new(0.78385901460875530441e-2) * t10006 + F::new(0.10136107947527008247e-3) * t10010 + F::new(0.14648281543675415196e-4) * t10014 + F::new(0.15176747947735985782e-5) * t10016 + F::new(0.55603792169291016668e-2) * t10019 - F::new(0.13913017666225690434e-3) * t10021;
    t11020
}
