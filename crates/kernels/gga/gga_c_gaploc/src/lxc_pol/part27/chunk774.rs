//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 774/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk774<F: Float>(t2580: F, t7305: F, t2549: F, t2564: F, t5508: F, t883: F, t732: F, t1877: F, t481: F, t941: F, t2042: F, t2558: F) -> (F, F, F, F, F) {
    let t7306 = t2580 * t7305;
    let t7309 = t2549 * t2564;
    let t7313 = t883 * t5508;
    let t7314 = t732 * t7313;
    let t7315 = t481 * t941 * t1877 * t7314;
    let t7317 = t2042 * t2558;
    (t7306, t7309, t7313, t7315, t7317)
}
