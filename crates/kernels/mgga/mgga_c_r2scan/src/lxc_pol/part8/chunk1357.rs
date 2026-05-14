//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1357/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1357<F: Float>(t1632: F, t551: F, t6528: F, t9937: F, t8198: F, t9177: F, t8629: F, t910: F, t10142: F, t10146: F, t20139: F, t20148: F, t20672: F, t2133: F, t2196: F, t24805: F, t2646: F, t29298: F, t29319: F, t3068: F, t31044: F, t32485: F, t495: F, t5109: F, t552: F, t6205: F, t7566: F, t9327: F, t948: F) -> (F, F) {
    let t33156 = t6528 * t551 * t1632 * t9937;
    let t33160 = t8198 * t9177;
    let t33168 = t910 * t8629;
    let t33182 = 0.1713958891116262235e0 * t20139 - t24805 + 0.69345773920434148507e1 * t33156 - 0.41530324072742201648e-1 * t20148 - 0.12713391885412927226e1 * t29298 - 0.69345773920434148504e0 * t33160 + 0.26004665220162805689e0 * t6205 * t10146 - 0.2600466522016280569e0 * t20672 * t10142 - 0.13002332610081402845e0 * t7566 * t3068 + 0.15602799132097683414e1 * t2196 * t551 * t552 * t33168 - 0.13002332610081402845e0 * t31044 * t948 - 0.13002332610081402845e0 * t9327 * t2646 - 0.83214928704520978208e1 * t29319 + 0.13002332610081402845e0 * t2133 * t5109 * t32485 * t495;
    (t33168, t33182)
}
