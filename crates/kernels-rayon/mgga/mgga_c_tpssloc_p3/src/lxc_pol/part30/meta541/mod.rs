//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1889;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1890;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta541(t26431: f64, t26470: f64, t1378: f64, t7696: f64, t794: f64, t6897: f64, t225: f64, t7704: f64, t1385: f64, t7749: f64, t3887: f64, t1375: f64, t1386: f64, t16022: f64, t16030: f64, t1843: f64, t2016: f64, t22670: f64, t22676: f64, t26366: f64, t26371: f64, t3758: f64, t3882: f64, t5326: f64, t6958: f64, t7750: f64, t16439: f64, t22656: f64, t22907: f64, t22909: f64, t22921: f64, t22924: f64, t22926: f64, t22928: f64, t22940: f64, t5215: f64, t5321: f64, t5354: f64, t6963: f64, t6993: f64, t7729: f64, t26223: f64, t26364: f64, t533: f64, t1390: f64, t1983: f64, t1393: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1869: f64, t22461: f64, t26103: f64, t26157: f64, t26166: f64, t26170: f64, t26178: f64, t26181: f64, t26183: f64, t4037: f64, t5107: f64, t6515: f64, t6517: f64, t6862: f64, t6872: f64, t7681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26471, t26472, t26474, t26475, t26477, t26482, t26485) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1889(t26431, t26470, t1378, t7696, t794, t6897, t225, t7704, t1385, t7749, t3887, t1375, t1386, t16022, t16030, t1843, t2016, t22670, t22676, t26366, t26371, t3758, t3882, t5326, t6958, t7750);
        let t26500 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1890(t16439, t1843, t2016, t22656, t22907, t22909, t22921, t22924, t22926, t22928, t22940, t3758, t5215, t5321, t5354, t6958, t6963, t6993, t7729);
        let (t26502, t26503, t26504, t26507) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1891(t26223, t26364, t26485, t26500, t533, t1390, t1983, t1393, t1442, t1459, t1774, t1849, t1869, t22461, t26103, t26157, t26166, t26170, t26178, t26181, t26183, t4037, t5107, t6515, t6517, t6862, t6872, t7681);
    (t26471, t26472, t26474, t26475, t26477, t26482, t26502, t26503, t26504, t26507)
}
