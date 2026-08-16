//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1578;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta409<F: Float>(t486: F, t6224: F, t11721: F, t1215: F, t4582: F, t4978: F, t1222: F, t6170: F, t6158: F, t6165: F, t11644: F, t11649: F, t11719: F, t11728: F, t15446: F, t15448: F, t15450: F, t15452: F, t15503: F, t15507: F, t18297: F, t488: F, t4974: F, t4980: F, t4984: F, t5005: F, t5416: F, t972: F, t135: F, t6187: F, t1174: F, t4889: F, t5040: F, t6183: F, t6177: F, t1198: F, t15484: F, t15488: F, t15490: F, t15494: F, t15498: F, t15524: F, t15550: F, t15574: F, t15580: F, t15737: F, t1748: F, t5024: F, t5030: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18300, t18301, t18303, t18307, t18316) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1578::<F>(t486, t6224, t11721, t1215, t4582, t4978, t1222, t6170, t6158, t6165, t11644, t11649, t11719, t11728, t15446, t15448, t15450, t15452, t15503, t15507, t18297, t488, t4974, t4980, t4984, t5005);
        let (t18321, t18324, t18329, t18332, t18337) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1579::<F>(t5416, t972, t135, t6187, t1174, t4889, t5040, t6183, t6177, t1198, t15484, t15488, t15490, t15494, t15498, t15524, t15550, t15574, t15580, t15737, t1748, t4980, t5024, t5030);
    (t18300, t18301, t18303, t18307, t18316, t18321, t18324, t18329, t18332, t18337)
}
