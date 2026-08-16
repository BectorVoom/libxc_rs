//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1453;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1454;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta359(t4300: f64, t865: f64, t2718: f64, t2684: f64, t4180: f64, t4181: f64, t9646: f64, t9647: f64, t2633: f64, t2645: f64, t4248: f64, t1496: f64, t9541: f64, t12850: f64, t12860: f64, t12861: f64, t12889: f64, t12891: f64, t12894: f64, t12906: f64, t12910: f64, t9457: f64, t9462: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t2427: f64, t4202: f64, t9869: f64, t2655: f64, t4205: f64, t12914: f64, t12922: f64, t12926: f64, t12927: f64, t12928: f64, t12934: f64, t12942: f64, t12944: f64, t12947: f64, t9724: f64, t9780: f64, t9789: f64, t9863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13071, t13072, t13076, t13080, t13084, t13087) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1453(t4300, t865, t2718, t2684, t4180, t4181, t9646, t9647, t2633, t2645, t4248, t1496, t9541);
        let t13093 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1454(t12850, t12860, t12861, t12889, t12891, t12894, t12906, t12910, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
        let (t13095, t13096, t13098, t13099) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1455(t2427, t4202, t9869, t2655, t4205, t12914, t12922, t12926, t12927, t12928, t12934, t12942, t12944, t12947, t9724, t9780, t9789, t9863);
    (t13071, t13072, t13076, t13080, t13084, t13087, t13093, t13095, t13096, t13098, t13099)
}
