//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 891/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk891<F: Float>(t1698: F, t1724: F, t1815: F, t639: F, t5024: F, t5522: F, t661: F, t1648: F, t4924: F, t1740: F, t1775: F, t5502: F, t7011: F) -> (F, F, F, F, F) {
    let t16921 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t639 * t1815 * t1698 * t1724;
    let t16925 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t639 * t5522 * t5024 * t661;
    let t16927 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t1648 * t4924;
    let t16928 = t1775 * t1740;
    let t16929 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t16928;
    let t16931 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7011 * t5502;
    (t16921, t16925, t16927, t16929, t16931)
}
