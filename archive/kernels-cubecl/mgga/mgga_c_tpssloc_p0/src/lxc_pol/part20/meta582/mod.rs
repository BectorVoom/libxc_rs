//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2150;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta582<F: Float>(t10216: F, t2978: F, t10479: F, t42333: F, t10922: F, t2960: F, t10489: F, t3048: F, t1041: F, t10868: F, t248: F, t2776: F, t3061: F, t676: F, t2771: F, t3129: F, t42742: F, t10962: F, t3103: F, t3078: F, t3082: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43317, t43322, t43325, t43332, t43336) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2150::<F>(t10216, t2978, t10479, t42333, t10922, t2960, t10489, t3048, t1041, t10868, t248, t2776);
        let (t43341, t43343, t43350, t43352, t43354) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2151::<F>(t3061, t676, t1041, t248, t2771, t3129, t42742, t10962, t3103, t3078, t3082, t3089);
    (t43317, t43322, t43325, t43332, t43336, t43341, t43343, t43350, t43352, t43354)
}
