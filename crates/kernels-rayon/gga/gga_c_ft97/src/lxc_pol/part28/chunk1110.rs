//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1110/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1110(t1701: f64, t26744: f64, t1008: f64, t355: f64, t53: f64, t7205: f64, t136847: f64, t138894: f64, t138899: f64, t138924: f64, t138927: f64, t138930: f64, t139009: f64, t139121: f64, t147412: f64, t147416: f64, t147425: f64, t147429: f64, t147432: f64, t147435: f64, t2001: f64, t23847: f64, t23866: f64, t32774: f64, t3380: f64, t3392: f64, t34857: f64, t34910: f64, t48841: f64, t7318: f64, t8812: f64, t8838: f64, t8859: f64, t935: f64) -> (f64, f64, f64) {
    let t147440 = t1701 * t26744;
    let t147445 = t7205 * t355 * t1008 * t53;
    let t147448 = t138894 + 0.24167761770734866964e0_f64 * t138899 - 0.1422705865505209271e0_f64 * t138924 - t138927 - 0.19592980390298668092e-1_f64 * t138930 + 0.64021763947734417195e0_f64 * t32774 * t136847 * t34910 + 0.30552173028732381488e-1_f64 * t2001 * t147412 - 0.15276086514366190744e-1_f64 * t3392 * t147416 + 0.20527106943485609994e0_f64 * t48841 * t34857 + 0.20527106943485609994e0_f64 * t8812 * t7318 * t3380 + 0.41054213886971219988e0_f64 * t8859 * t147425 + 0.10947790369858991997e2_f64 * t23866 * t147429 - 0.45306850413028723348e0_f64 * t23847 * t147432 + 0.45306850413028723348e0_f64 * t8838 * t147435 + 0.35314306798406949389e-2_f64 * t139009 * t935 - 0.45306850413028723348e0_f64 * t23847 * t147440 - 0.58778941170896004276e-1_f64 * t139121 * t147445;
    (t147440, t147445, t147448)
}
