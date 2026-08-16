//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 907/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk907<F: Float>(t10000: F, t10003: F, t10006: F, t10010: F, t10014: F, t10016: F, t10019: F, t10021: F, t9986: F, t9991: F, t9993: F, t9995: F, t9997: F) -> F {
    let t11020 = F::cast_from(0.29517957899305555558e-5_f64) * t9986 + F::cast_from(0.84410248952307505288e-7_f64) * t9991 - F::cast_from(0.2748593934505475288e-5_f64) * t9993 + F::cast_from(0.2162369695472428426e-1_f64) * t9995 + F::cast_from(0.56273499301538336858e-7_f64) * t9997 + F::cast_from(0.56273499301538336858e-7_f64) * t10000 - F::cast_from(0.27801896084645508334e-2_f64) * t10003 - F::cast_from(0.78385901460875530441e-2_f64) * t10006 + F::cast_from(0.10136107947527008247e-3_f64) * t10010 + F::cast_from(0.14648281543675415196e-4_f64) * t10014 + F::cast_from(0.15176747947735985782e-5_f64) * t10016 + F::cast_from(0.55603792169291016668e-2_f64) * t10019 - F::cast_from(0.13913017666225690434e-3_f64) * t10021;
    t11020
}
