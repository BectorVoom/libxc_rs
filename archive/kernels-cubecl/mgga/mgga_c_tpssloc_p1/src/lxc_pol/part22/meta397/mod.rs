//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta397<F: Float>(t135: F, t6187: F, t1174: F, t4889: F, t5040: F, t6183: F, t6177: F, t1198: F, t15484: F, t15488: F, t15490: F, t15494: F, t15498: F, t15524: F, t15550: F, t15574: F, t15580: F, t15737: F, t1748: F, t18321: F, t4980: F, t5024: F, t5030: F) -> (F, F, F, F, F, F, F, F) {
        let (t18324, t18325, t18327, t18329, t18330, t18332, t18333, t18337) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1690::<F>(t135, t6187, t1174, t4889, t5040, t6183, t6177, t1198, t15484, t15488, t15490, t15494, t15498, t15524, t15550, t15574, t15580, t15737, t1748, t18321, t4980, t5024, t5030);
    (t18324, t18325, t18327, t18329, t18330, t18332, t18333, t18337)
}
