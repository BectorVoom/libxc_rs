//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1315/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1315<F: Float>(t28873: F, t6963: F, t25462: F, t31348: F, t31551: F, t317: F, t31930: F, t870: F, t875: F, t1091: F, t111711: F, t112463: F, t112465: F, t112512: F, t112515: F, t1479: F, t1506: F, t19308: F, t2: F, t26: F, t2665: F, t28993: F, t29008: F, t31669: F, t4: F, t6210: F, t6216: F, t684: F, t82562: F) -> (F, F) {
    let t125769 = t6963 * t28873;
    let t125771 = t25462 * t31348;
    let t125777 = t31551 * t317;
    let t125782 = t31930 * t870;
    let t125783 = t125782 * t875;
    let t125791 = -t112463 - t112465 - t29008 * t28993 / 9.0 + t6210 * t31669 / 3.0 - t125769 / 9.0 + t125771 / 54.0 - t6216 * t2665 * t111711 * t1091 / 9.0 - t6216 * t2665 * t125777 * t684 / 18.0 - 2.0 * t125783 - t19308 * t1506 + t82562 * t2 * t4 * t26 * t1479 / 6.0 + t112512 + t112515;
    (t125783, t125791)
}
