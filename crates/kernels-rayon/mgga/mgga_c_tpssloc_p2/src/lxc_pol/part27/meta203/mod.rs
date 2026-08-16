//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1016;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1017;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta203(t4528: f64, t973: f64, t1597: f64, t2987: f64, t2990: f64, t2824: f64, t3003: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t340: f64, t343: f64, t974: f64, t984: f64, t1593: f64, t1600: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2986: f64, t4507: f64, t4511: f64, t4515: f64, t4519: f64, t4523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4529, t4531, t4532, t4540) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1016(t4528, t973, t1597, t2987, t2990, t2824, t3003, t4384, t4387, t4390, t4393);
        let (t4541, t4542) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1017(t340, t4540, t343);
        let (t4543, t4546, t4547, t4548, t4549, t4552) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1018(t4542, t974, t340, t1597, t984, t343, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t973);
    (t4531, t4532, t4540, t4541, t4542, t4543, t4546, t4547, t4548, t4549, t4552)
}
