//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1980;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1981;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta565(t1393: f64, t1459: f64, t1849: f64, t24932: f64, t26166: f64, t26170: f64, t26178: f64, t26181: f64, t26183: f64, t26505: f64, t27879: f64, t27888: f64, t27903: f64, t4037: f64, t4073: f64, t4077: f64, t574: f64, t652: f64, t7266: f64, t7412: f64, t8107: f64, t27860: f64, t27867: f64, t27878: f64, t3: f64, t112: f64, t8110: f64, t1458: f64, t24969: f64, t24972: f64, t26533: f64, t26535: f64, t26537: f64, t26539: f64, t26541: f64, t26544: f64, t26547: f64, t26549: f64, t26552: f64, t26554: f64, t4072: f64, t5376: f64, t577: f64, t671: f64, t7423: f64, t3701: f64, t6995: f64, t7752: f64, t1390: f64, t22811: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t12019: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t27905 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1980(t1393, t1459, t1849, t24932, t26166, t26170, t26178, t26181, t26183, t26505, t27879, t27888, t27903, t4037, t4073, t4077, t574, t652, t7266, t7412, t8107);
        let (t27907, t27908, t27921, t27930) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1981(t27860, t27867, t27878, t27905, t3, t112, t8110, t1458, t24969, t24972, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7423);
        let (t31035, t33136, t34475, t39041, t39049, t39054, t39063, t40590) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1982(t3701, t6995, t7752, t1390, t22811, t2233, t2239, t601, t9238, t85, t24, t12019, t566);
    (t27907, t27908, t27921, t27930, t31035, t33136, t34475, t39041, t39049, t39054, t39063, t40590)
}
