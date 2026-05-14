//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1003/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1003<F: Float>(t25622: F, t2721: F, t5025: F, t2742: F, t2778: F, t5016: F, t2693: F, t4983: F, t10594: F, t4054: F, t5075: F, t7274: F, t999: F, t1773: F, t5107: F) -> (F, F, F, F, F, F) {
    let t43003 = t2721 * t25622 * t5025;
    let t43112 = t2778 * t2742 * t5016;
    let t43210 = t4983 * t2693;
    let t43260 = t4054 * t10594;
    let t43347 = t999 * t7274 * t5075;
    let t43414 = t1773 * t5107;
    (t43003, t43112, t43210, t43260, t43347, t43414)
}
