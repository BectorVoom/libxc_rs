//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1247/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1247<F: Float>(t5218: F, t1094: F, t1102: F, t2916: F, t3058: F, t3061: F, t15142: F, t18187: F, t15078: F, t15083: F, t17627: F, t18184: F, t18191: F, t4297: F, t5229: F, t53793: F, t53812: F, t53823: F, t53826: F, t53829: F, t53831: F) -> (F, F, F, F) {
    let t58880 = t5218 * t5218;
    let t58884 = 0.35089340384731224426e1 * t1102 * t2916 * t58880 * t1094;
    let t58888 = 0.51947267698127589897e2 * t1102 * t3058 * t58880 * t3061;
    let t58889 = t15142 * t18187;
    let t58905 = t58884 - t58888 - 400.0 / 27.0 * t4297 * t58889 + 80000.0 / 243.0 * t53823 + 200.0 / 81.0 * t53826 - 400.0 / 9.0 * t15083 * t18191 + 8.0 / 9.0 * t53829 + 80000.0 / 81.0 * t53812 * t17627 - 200.0 / 3.0 * t53831 * t5229 - 1520000.0 / 243.0 * t53793 * t17627 + 8.0 / 3.0 * t15078 * t18184;
    (t58880, t58884, t58888, t58905)
}
