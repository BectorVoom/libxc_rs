//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2123/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2123(t86895: f64, t23035: f64, t23241: f64, t25224: f64, t7480: f64, t81632: f64, t22975: f64, t23191: f64, t25184: f64, t25330: f64, t2597: f64, t2713: f64, t4147: f64, t4268: f64, t86844: f64, t86847: f64, t86852: f64, t86857: f64, t86862: f64, t86866: f64, t86869: f64, t86870: f64, t86875: f64, t86881: f64, t86884: f64, t86887: f64, t86891: f64) -> f64 {
    let t86896 = 0.16449340668482264365e-1_f64 * t86895;
    let t86901 = t23035 * t25224 * t23241;
    let t86903 = t81632 * t7480;
    let t86905 = t86844 + 0.16449340668482264365e-1_f64 * t86847 + 0.3289868133696452873e-1_f64 * t86852 + 0.3289868133696452873e-1_f64 * t86857 + 0.3289868133696452873e-1_f64 * t86862 + 0.16449340668482264365e-1_f64 * t86866 + t86869 - 0.52089578783527170489e-1_f64 * t86870 + 0.3289868133696452873e-1_f64 * t86875 + 2.0_f64 * t4147 * t22975 - 0.49348022005446793095e-1_f64 * t86881 + 0.3289868133696452873e-1_f64 * t86884 + t86887 + 4.0_f64 * t2597 * t25184 - 0.16449340668482264365e-1_f64 * t86891 + t86896 - 2.0_f64 * t2713 * t25330 - t4268 * t23191 + 0.49348022005446793095e-1_f64 * t86901 - 0.12793931631041761173e0_f64 * t86903;
    t86905
}
