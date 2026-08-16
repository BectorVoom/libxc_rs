//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1622;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1623;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta420(t1216: f64, t18300: f64, t4582: f64, t5001: f64, t5018: f64, t1730: f64, t5023: f64, t1177: f64, t18225: f64, t1193: f64, t6109: f64, t248: f64, t3570: f64, t6230: f64, t3515: f64, t1230: f64, t18241: f64, t11546: f64, t18206: f64, t11738: f64, t1174: f64, t1218: f64, t1227: f64, t1232: f64, t15591: f64, t15594: f64, t15754: f64, t1737: f64, t1748: f64, t3490: f64, t4889: f64, t5002: f64, t5005: f64, t5014: f64, t5030: f64, t5033: f64, t6207: f64, t6211: f64, t18316: f64, t18337: f64, t18390: f64, t18951: f64, t18989: f64, t19029: f64, t19075: f64, t466: f64, t5068: f64, t6260: f64, t18940: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19077, t19080, t19083, t19087, t19090, t19095) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1622(t1216, t18300, t4582, t5001, t5018, t1730, t5023, t1177, t18225, t1193, t6109, t248, t3570, t6230);
        let (t19101, t19117) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1623(t19095, t3515, t1230, t18241, t248, t11546, t18206, t11738, t1174, t1218, t1227, t1232, t15591, t15594, t15754, t1737, t1748, t19077, t19080, t19083, t19087, t19090, t3490, t4889, t5002, t5005, t5014, t5030, t5033, t6207, t6211);
        let (t19120, t19121, t19123, t19128) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1624(t18316, t18337, t18390, t18951, t18989, t19029, t19075, t19117, t466, t5068, t6260, t18940, t491);
    (t19077, t19095, t19101, t19120, t19121, t19123, t19128)
}
