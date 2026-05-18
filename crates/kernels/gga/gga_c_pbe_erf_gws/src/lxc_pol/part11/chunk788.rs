//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 788/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk788<F: Float>(t10887: F, t10889: F, t1024: F, t10419: F, t11005: F, t950: F, t5548: F, t587: F, t10505: F, t954: F, t1815: F, t639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12774 = F::new(4.0) / F::new(15.0) * t10887;
    let t12775 = F::new(8.0) / F::new(15.0) * t10889;
    let t12777 = F::new(4.0) / F::new(5.0) * t10419 * t1024;
    let t12778 = t11005 * t950;
    let t12779 = t5548 * t12778;
    let t12781 = F::new(8.0) / F::new(15.0) * t587 * t12779;
    let t12782 = t10505 * t954;
    let t12783 = t1815 * t12782;
    let t12785 = F::new(4.0) / F::new(15.0) * t639 * t12783;
    (t12774, t12775, t12777, t12778, t12779, t12781, t12782, t12783, t12785)
}
