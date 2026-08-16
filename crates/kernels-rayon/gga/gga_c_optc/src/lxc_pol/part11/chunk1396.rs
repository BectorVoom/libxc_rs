//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1396/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1396(t5218: f64, t1094: f64, t1102: f64, t2916: f64, t3058: f64, t3061: f64, t15142: f64, t18187: f64, t15078: f64, t15083: f64, t17627: f64, t18184: f64, t18191: f64, t4297: f64, t5229: f64, t53793: f64, t53812: f64, t53823: f64, t53826: f64, t53829: f64, t53831: f64) -> (f64, f64, f64, f64) {
    let t58880 = t5218 * t5218;
    let t58884 = 0.35089340384731224426e1_f64 * t1102 * t2916 * t58880 * t1094;
    let t58888 = 0.51947267698127589897e2_f64 * t1102 * t3058 * t58880 * t3061;
    let t58889 = t15142 * t18187;
    let t58905 = t58884 - t58888 - 400.0_f64 / 27.0_f64 * t4297 * t58889 + 80000.0_f64 / 243.0_f64 * t53823 + 200.0_f64 / 81.0_f64 * t53826 - 400.0_f64 / 9.0_f64 * t15083 * t18191 + 8.0_f64 / 9.0_f64 * t53829 + 80000.0_f64 / 81.0_f64 * t53812 * t17627 - 200.0_f64 / 3.0_f64 * t53831 * t5229 - 1520000.0_f64 / 243.0_f64 * t53793 * t17627 + 8.0_f64 / 3.0_f64 * t15078 * t18184;
    (t58880, t58884, t58888, t58905)
}
