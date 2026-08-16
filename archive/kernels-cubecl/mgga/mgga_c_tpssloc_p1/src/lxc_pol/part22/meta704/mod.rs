//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta704<F: Float>(t1222: F, t18982: F, t13969: F, t18947: F, t3506: F, t11719: F, t18302: F, t1174: F, t18225: F, t3431: F, t18221: F, t15522: F, t4889: F, t3545: F, t6109: F, t19071: F, t3515: F, t11728: F, t18306: F, t11738: F, t19076: F, t18940: F, t486: F, t15753: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t66410, t66413, t66437, t66449, t66452, t66458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292::<F>(t1222, t18982, t13969, t18947, t3506, t11719, t18302, t1174, t18225, t3431, t18221, t15522, t4889);
        let (t66500, t66512, t66515, t66518, t66533, t66545) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2293::<F>(t3545, t6109, t13969, t19071, t3515, t11728, t18306, t11738, t19076, t18940, t486, t15753, t4889);
    (t66410, t66413, t66437, t66449, t66452, t66458, t66500, t66512, t66515, t66518, t66533, t66545)
}
