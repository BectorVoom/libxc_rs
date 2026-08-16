//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1946;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta623(t16235: f64, t91361: f64, t5303: f64, t80820: f64, t16356: f64, t6916: f64, t16018: f64, t1998: f64, t236: f64, t6926: f64, t1339: f64, t54153: f64, t550: f64, t6936: f64, t16311: f64, t3788: f64, t3850: f64, t57554: f64, t26233: f64, t3858: f64, t22783: f64, t5310: f64, t22760: f64, t5234: f64, t3795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91362, t91364, t91366, t91370, t91374) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1946(t16235, t91361, t5303, t80820, t16356, t6916, t16018, t1998, t236, t6926, t1339, t54153, t550, t6936);
        let (t91378, t91381, t91384, t91386, t91389) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1947(t16311, t3788, t3850, t6936, t57554, t26233, t3858, t22783, t5310, t22760, t5234, t3795);
    (t91362, t91364, t91366, t91370, t91374, t91378, t91381, t91384, t91386, t91389)
}
