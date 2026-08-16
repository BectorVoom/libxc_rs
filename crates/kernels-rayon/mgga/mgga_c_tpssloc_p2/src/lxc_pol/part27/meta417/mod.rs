//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1721;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta417(t4034: f64, t6535: f64, t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64, t2332: f64, t2358: f64, t6530: f64, t109: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t22467, t22469, t22470, t22471, t22472, t22473, t22474, t22476) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1721(t4034, t6535, t107, t240, t625, t656, t666, t2331, t63, t2332, t2358, t6530);
        let t22479 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1722(t109, t22469, t22472, t22474, t22476);
    (t22467, t22469, t22470, t22471, t22473, t22479)
}
