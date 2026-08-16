//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta703(t15689: f64, t4889: f64, t1174: f64, t135: f64, t18996: f64, t15743: f64, t5024: f64, t18363: f64, t3577: f64, t45124: f64, t11697: f64, t18359: f64, t15572: f64, t15740: f64, t18382: f64, t1215: f64, t6224: f64, t1227: f64, t13969: f64, t18954: f64, t19067: f64, t1222: f64, t18297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66273, t66276, t66324, t66334, t66337) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290(t15689, t4889, t1174, t135, t18996, t15743, t5024, t18363, t3577, t45124, t11697, t18359);
        let (t66360, t66363, t66388, t66398, t66406, t66408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291(t15572, t15740, t11697, t18382, t3577, t1215, t6224, t1227, t13969, t18954, t19067, t1222, t18297);
    (t66273, t66276, t66324, t66334, t66337, t66360, t66363, t66388, t66398, t66406, t66408)
}
