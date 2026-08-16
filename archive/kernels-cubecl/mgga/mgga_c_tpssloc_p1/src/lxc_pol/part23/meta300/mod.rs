//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta300<F: Float>(t21510: F, t4588: F, t4582: F, t10970: F, t21130: F, t248: F, t1616: F, t5681: F, t3071: F, t1539: F, t5873: F, t10403: F, t1041: F, t13966: F, t13995: F, t17621: F, t17625: F, t17656: F, t17660: F, t17662: F, t17668: F, t21503: F, t3039: F, t3070: F, t5909: F) -> (F, F, F, F, F, F, F, F) {
        let (t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1029::<F>(t21510, t4588, t4582, t10970, t21130, t248, t1616, t5681, t3071, t1539, t5873, t10403, t1041, t13966, t13995, t17621, t17625, t17656, t17660, t17662, t17668, t21503, t3039, t3070, t5909);
    (t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529)
}
