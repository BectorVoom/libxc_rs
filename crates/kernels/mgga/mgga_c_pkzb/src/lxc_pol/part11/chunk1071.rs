//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1071/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1071<F: Float>(t16592: F, t28954: F, t28955: F, t28956: F, t28957: F, t28958: F, t28959: F, t28960: F, t28961: F, t28962: F, t28963: F, t16600: F, t16615: F, t16617: F, t19754: F, t10666: F, t1769: F) -> (F, F, F, F, F, F) {
    let t28964 = t28954 - t28955 - t28956 - t28957 - t28958 + t28959 + t28960 + t28961 - t16592 - t28962 - t28963;
    let t28966 = 0.32530743900905219526e-1 * t16600;
    let t28967 = 0.10389515463408878255e3 * t16615;
    let t28968 = 0.10254018858216406658e4 * t16617;
    let t28970 = 72.0 * t19754;
    let t28977 = t1769 * t10666;
    (t28964, t28966, t28967, t28968, t28970, t28977)
}
