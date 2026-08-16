//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1432/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1432<F: Float>(t35825: F, t35834: F, t35887: F, t44077: F, t54941: F, t54944: F, t54947: F, t54989: F, t54999: F, t55001: F, t55004: F, t55011: F, t55021: F, t55024: F, t55027: F) -> F {
    let t59804 = -F::cast_from(0.15146801702008125515e1_f64) * t44077 + F::cast_from(0.33037286659193699704e3_f64) * t54941 + F::cast_from(0.46885265819954330464e4_f64) * t54944 - F::cast_from(0.24951672488470492992e3_f64) * t54947 + F::cast_from(0.69310201356862480534e1_f64) * t35825 + F::cast_from(0.16829779668897917239e1_f64) * t35834 + F::cast_from(0.42929192542166705456e-1_f64) * t35887 + F::cast_from(0.13613985915860191978e1_f64) * t54989 + F::cast_from(0.42991534471137448352e0_f64) * t54999 + F::cast_from(0.61818037260720055856e0_f64) * t55001 + F::cast_from(0.18583473745796456084e3_f64) * t55004 - F::cast_from(0.1343485452223045261e0_f64) * t55011 - F::cast_from(0.30909018630360027928e0_f64) * t55021 + F::cast_from(0.38636273287950034909e-1_f64) * t55024 - F::cast_from(0.17581974682482873924e4_f64) * t55027;
    t59804
}
