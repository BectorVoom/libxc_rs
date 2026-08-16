//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1855;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta522(t26431: f64, t26470: f64, t1378: f64, t7696: f64, t794: f64, t6897: f64, t225: f64, t7704: f64, t1385: f64, t7749: f64, t3887: f64, t1375: f64, t1386: f64, t16022: f64, t16030: f64, t1843: f64, t2016: f64, t22670: f64, t22676: f64, t26366: f64, t26371: f64, t3758: f64, t3882: f64, t5326: f64, t6958: f64, t7750: f64, t16439: f64, t22656: f64, t22907: f64, t22909: f64, t22921: f64, t22924: f64, t22926: f64, t22928: f64, t22940: f64, t5215: f64, t5321: f64, t5354: f64, t6963: f64, t6993: f64, t7729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26471, t26472, t26474, t26475, t26477, t26482, t26485) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1855(t26431, t26470, t1378, t7696, t794, t6897, t225, t7704, t1385, t7749, t3887, t1375, t1386, t16022, t16030, t1843, t2016, t22670, t22676, t26366, t26371, t3758, t3882, t5326, t6958, t7750);
        let t26500 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1856(t16439, t1843, t2016, t22656, t22907, t22909, t22921, t22924, t22926, t22928, t22940, t3758, t5215, t5321, t5354, t6958, t6963, t6993, t7729);
    (t26471, t26472, t26474, t26475, t26477, t26482, t26485, t26500)
}
