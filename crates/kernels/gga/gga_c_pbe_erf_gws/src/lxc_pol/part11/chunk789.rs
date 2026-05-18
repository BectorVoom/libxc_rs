//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 789/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk789<F: Float>(t10914: F, t10925: F, t10928: F, t10931: F, t10933: F, t10973: F, t10993: F, t7811: F, t12476: F, t1821: F, t587: F, t12468: F, t2559: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12786 = F::new(16.0) / F::new(15.0) * t10914;
    let t12787 = F::new(8.0) / F::new(27.0) * t10925;
    let t12788 = F::new(8.0) / F::new(45.0) * t10928;
    let t12789 = F::new(16.0) / F::new(15.0) * t10931;
    let t12790 = F::new(16.0) / F::new(45.0) * t10933;
    let t12791 = F::new(8.0) / F::new(15.0) * t10973;
    let t12792 = F::new(16.0) / F::new(45.0) * t10993;
    let t12793 = F::new(4.0) / F::new(45.0) * t7811;
    let t12794 = t1821 * t12476;
    let t12796 = F::new(8.0) / F::new(15.0) * t587 * t12794;
    let t12797 = t2559 * t12468;
    (t12786, t12787, t12788, t12789, t12790, t12791, t12792, t12793, t12794, t12796, t12797)
}
