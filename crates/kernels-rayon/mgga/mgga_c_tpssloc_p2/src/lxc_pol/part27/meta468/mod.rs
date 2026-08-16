//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1826;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta468(t23394: f64, t3175: f64, t6704: f64, t1922: f64, t3010: f64, t2776: f64, t6690: f64, t6689: f64, t1945: f64, t3020: f64, t6768: f64, t990: f64, t2250: f64, t3: f64, t1933: f64, t368: f64, t3068: f64, t1058: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23395, t23396, t23399, t23402, t23403, t23408, t23410) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1826(t23394, t3175, t6704, t1922, t3010, t2776, t6690, t6689, t1945, t3020, t6768, t990);
        let (t23413, t23414, t23417, t23418, t23419) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1827(t2250, t3, t1933, t368, t3068, t1058, sigma0);
    (t23395, t23396, t23399, t23402, t23403, t23408, t23410, t23413, t23414, t23417, t23418, t23419)
}
