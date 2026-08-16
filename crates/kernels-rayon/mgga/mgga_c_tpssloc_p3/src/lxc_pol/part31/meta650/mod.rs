//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta650(t17037: f64, t1888: f64, t22996: f64, t232: f64, t58204: f64, t6646: f64, t2632: f64, t58166: f64, t28423: f64, t6579: f64, t28427: f64, t25038: f64, t25248: f64, t25249: f64, t4119: f64, t28419: f64, t23035: f64, t23153: f64, t5527: f64, t6637: f64, t22893: f64, t28341: f64, t81640: f64, t1484: f64, t6552: f64, t87586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98478, t98482, t98486, t98488, t98490, t98502) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1926(t17037, t1888, t22996, t232, t58204, t6646, t2632, t58166, t28423, t6579, t28427, t25038, t25248, t25249, t4119);
        let (t98505, t98513, t98516, t98520) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1927(t28419, t6579, t23035, t23153, t5527, t6637, t22893, t28341, t81640, t1484, t6552, t87586);
    (t98478, t98482, t98486, t98488, t98490, t98502, t98505, t98513, t98516, t98520)
}
