//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1219/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1219<F: Float>(t3324: F, t944: F, t1105: F, t13751: F, t13756: F, t14149: F, t14153: F, t14390: F, t2051: F, t2074: F, t30098: F, t3189: F, t3946: F, t4062: F, t50825: F, t50837: F, t50846: F, t52801: F, t52810: F, t52812: F, t52816: F, t52821: F, t52823: F) -> F {
    let t52829 = t3324 * t944;
    let t52833 = F::cast_from(3.0_f64) * t1105 * t3946 * t50825 + F::cast_from(12.0_f64) * t13751 * t13756 * t3189 - F::cast_from(2.0_f64) * t14149 * t3324 * t4062 + F::cast_from(4.0_f64) * t14153 * t4062 * t52829 + F::cast_from(3.0_f64) * t14390 * t2074 * t3946 + F::cast_from(2.0_f64) * t2051 * t4062 * t52816 - F::cast_from(12.0_f64) * t30098 * t52823 + F::cast_from(2.0_f64) * t50837 - t50846 + t52801 - t52810 - t52812 - t52821;
    t52833
}
