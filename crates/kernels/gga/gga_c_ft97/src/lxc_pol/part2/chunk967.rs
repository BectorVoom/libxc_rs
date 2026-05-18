//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 967/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk967<F: Float>(t1232: F, t1771: F, t4224: F, t458: F, t11717: F, t4210: F, t10261: F, t2682: F, t4218: F, t2681: F, t2739: F, t1228: F, t8282: F) -> (F, F, F, F, F, F) {
    let t15011 = t1771 * t1232;
    let t15014 = F::new(2.0) / F::new(3.0) * t458 * t4224;
    let t15015 = t11717 * t4210;
    let t15018 = t10261 * t4218 * t2682;
    let t15022 = t2681 * t4218 * t2739;
    let t15025 = t8282 * t1228;
    (t15011, t15014, t15015, t15018, t15022, t15025)
}
