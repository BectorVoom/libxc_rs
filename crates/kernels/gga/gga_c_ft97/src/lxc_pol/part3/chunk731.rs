//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 731/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk731<F: Float>(t10580: F, t2: F, t1775: F, t4215: F, t1232: F, t1771: F, t4224: F, t458: F, t11717: F, t4210: F, t1228: F, t8282: F) -> (F, F, F, F, F, F) {
    let t14961 = t10580 * t2;
    let t14999 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t4215;
    let t15011 = t1771 * t1232;
    let t15014 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t458 * t4224;
    let t15015 = t11717 * t4210;
    let t15025 = t8282 * t1228;
    (t14961, t14999, t15011, t15014, t15015, t15025)
}
