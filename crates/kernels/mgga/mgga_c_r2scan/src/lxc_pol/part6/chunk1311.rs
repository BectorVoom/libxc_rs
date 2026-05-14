//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1311/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1311<F: Float>(t2558: F, t5147: F, t5148: F, t2651: F, t6345: F, t1584: F, t8165: F, t2252: F, t2567: F, t2148: F, t6165: F, t1234: F, t7614: F, t2155: F, t24714: F, t8088: F) -> (F, F, F, F, F, F) {
    let t24774 = t5147 * t5148 * t2558;
    let t24776 = t2651 * t6345;
    let t24777 = 0.12713391885412927226e1 * t24776;
    let t24778 = t1584 * t8165;
    let t24786 = t2567 * t2252;
    let t24788 = t6165 * t2148 * t24786;
    let t24790 = t2567 * t1234;
    let t24792 = t7614 * t2148 * t24790;
    let t24795 = t2155 * t8088 * t24714;
    (t24774, t24777, t24778, t24788, t24792, t24795)
}
