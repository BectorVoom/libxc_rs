//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1820;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta465(t221: f64, t2987: f64, t1926: f64, t344: f64, t381: f64, t225: f64, t1054: f64, t883: f64, t1065: f64, t607: f64, t6733: f64, t6691: f64, t1955: f64, t3175: f64, t10165: f64, t6686: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23326, t23327) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1819(t221, t2987, t1926);
        let (t23328, t23329) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1820(t344, t381, t225);
        let (t23330, t23331, t23332, t23333, t23336, t23337, t23341, t23346) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1821(t1054, t883, t1065, t607, t23329, t381, t6733, t6691, t1955, t3175, t10165, t6686, t6712);
    (t23326, t23327, t23328, t23329, t23330, t23331, t23332, t23333, t23336, t23337, t23341, t23346)
}
