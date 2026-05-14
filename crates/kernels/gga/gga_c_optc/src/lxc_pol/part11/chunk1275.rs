//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1275/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1275<F: Float>(t35825: F, t35834: F, t35887: F, t44077: F, t54941: F, t54944: F, t54947: F, t54989: F, t54999: F, t55001: F, t55004: F, t55011: F, t55021: F, t55024: F, t55027: F, t1: F, t4570: F) -> (F, F) {
    let t59804 = -0.15146801702008125515e1 * t44077 + 0.33037286659193699704e3 * t54941 + 0.46885265819954330464e4 * t54944 - 0.24951672488470492992e3 * t54947 + 0.69310201356862480534e1 * t35825 + 0.16829779668897917239e1 * t35834 + 0.42929192542166705456e-1 * t35887 + 0.13613985915860191978e1 * t54989 + 0.42991534471137448352e0 * t54999 + 0.61818037260720055856e0 * t55001 + 0.18583473745796456084e3 * t55004 - 0.1343485452223045261e0 * t55011 - 0.30909018630360027928e0 * t55021 + 0.38636273287950034909e-1 * t55024 - 0.17581974682482873924e4 * t55027;
    let t59822 = t4570 * t1;
    (t59804, t59822)
}
