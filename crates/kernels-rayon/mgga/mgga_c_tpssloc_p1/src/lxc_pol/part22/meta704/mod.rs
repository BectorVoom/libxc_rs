//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta704(t1222: f64, t18982: f64, t13969: f64, t18947: f64, t3506: f64, t11719: f64, t18302: f64, t1174: f64, t18225: f64, t3431: f64, t18221: f64, t15522: f64, t4889: f64, t3545: f64, t6109: f64, t19071: f64, t3515: f64, t11728: f64, t18306: f64, t11738: f64, t19076: f64, t18940: f64, t486: f64, t15753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66410, t66413, t66437, t66449, t66452, t66458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292(t1222, t18982, t13969, t18947, t3506, t11719, t18302, t1174, t18225, t3431, t18221, t15522, t4889);
        let (t66500, t66512, t66515, t66518, t66533, t66545) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2293(t3545, t6109, t13969, t19071, t3515, t11728, t18306, t11738, t19076, t18940, t486, t15753, t4889);
    (t66410, t66413, t66437, t66449, t66452, t66458, t66500, t66512, t66515, t66518, t66533, t66545)
}
