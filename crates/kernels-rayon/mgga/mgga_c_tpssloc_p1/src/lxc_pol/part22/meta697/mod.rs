//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2279;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta697(t18392: f64, t3490: f64, t1227: f64, t18241: f64, t248: f64, t3521: f64, t19040: f64, t15734: f64, t5024: f64, t11818: f64, t3515: f64, t6230: f64, t15578: f64, t4889: f64, t11789: f64, t5979: f64, t19051: f64, t3523: f64, t19080: f64, t3572: f64, t11709: f64, t18356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65613, t65617, t65619, t65628, t65632) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2279(t18392, t3490, t1227, t18241, t248, t3521, t19040, t15734, t5024, t11818, t3515, t6230);
        let (t65637, t65647, t65649, t65651, t65660) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280(t15578, t4889, t11789, t1227, t248, t5979, t19051, t3523, t19080, t3572, t11709, t18356);
    (t65613, t65617, t65619, t65628, t65632, t65637, t65647, t65649, t65651, t65660)
}
