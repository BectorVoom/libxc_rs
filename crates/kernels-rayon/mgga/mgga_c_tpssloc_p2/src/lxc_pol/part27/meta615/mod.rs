//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta615(t23460: f64, t995: f64, t23452: f64, t6739: f64, t6741: f64, t23482: f64, t23488: f64, t23508: f64, t6721: f64, t1937: f64, t23453: f64, t40: f64, t23476: f64, t23479: f64, t6722: f64, t23563: f64, t6740: f64, t6747: f64, t23422: f64, t3139: f64, t10922: f64, t6717: f64, t10993: f64, t10981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83098, t83111, t83114, t83120, t83127) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2091(t23460, t995, t23452, t6739, t6741, t23482, t23488, t23508, t6721, t1937, t23453, t40);
        let (t83134, t83138, t83139, t83153, t83157, t83159, t83165) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2092(t23476, t23479, t6722, t23563, t6740, t6747, t23422, t3139, t10922, t6717, t10993, t10981);
    (t83098, t83111, t83114, t83120, t83127, t83134, t83138, t83139, t83153, t83157, t83159, t83165)
}
