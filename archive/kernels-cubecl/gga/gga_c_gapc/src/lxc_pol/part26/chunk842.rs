//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 842/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk842<F: Float>(t9775: F, t9777: F, t9780: F, t9783: F, t9789: F, t9791: F, t9793: F, t9796: F, t9800: F, t9802: F, t9805: F, t9808: F, t9811: F) -> F {
    let t9813 = F::cast_from(0.61644410594352107859e-7_f64) * t9775 + F::cast_from(0.4637672555408563478e-4_f64) * t9777 + F::cast_from(0.38647271295071362318e-6_f64) * t9780 - F::cast_from(0.687148483626368822e-6_f64) * t9783 - F::cast_from(0.2813674965076916843e-8_f64) * t9789 - F::cast_from(0.4637672555408563478e-4_f64) * t9791 + F::cast_from(0.66340671383216596998e-6_f64) * t9793 - F::cast_from(0.27801896084645508334e-2_f64) * t9796 - F::cast_from(0.14758978949652777778e-5_f64) * t9800 - F::cast_from(0.13492128925537291361e-5_f64) * t9802 - F::cast_from(0.7588373973867992891e-7_f64) * t9805 + F::cast_from(0.13492128925537291361e-6_f64) * t9808 - F::cast_from(0.28985453471303521736e-5_f64) * t9811;
    t9813
}
