//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta564(t1519: f64, t794: f64, t23164: f64, t6555: f64, t23035: f64, t23241: f64, t25224: f64, t7480: f64, t81632: f64, t25038: f64, t25040: f64, t82159: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t22986: f64, t23270: f64, t25191: f64, t2742: f64, t25059: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86893, t86895, t86901, t86903, t86909) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1838(t1519, t794, t23164, t6555, t23035, t23241, t25224, t7480, t81632, t25038, t25040, t82159);
        let (t86911, t86916, t86923, t86928) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1839(t23030, t25035, t23228, t7479, t81573, t22986, t23270, t25191, t2742, t25059, t6562, t794);
    (t86893, t86895, t86901, t86903, t86909, t86911, t86916, t86923, t86928)
}
