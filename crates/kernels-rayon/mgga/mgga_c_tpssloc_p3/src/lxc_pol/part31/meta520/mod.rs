//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1728;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1729;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta520(t2039: f64, t6287: f64, t2075: f64, t5493: f64, t1774: f64, t7801: f64, t19596: f64, t2095: f64, t1268: f64, t1458: f64, t19451: f64, t27188: f64, t28002: f64, t28007: f64, t28943: f64, t28951: f64, t28959: f64, t4028: f64, t7042: f64, t7676: f64, t20085: f64, t24432: f64, t28830: f64, t23957: f64, t28826: f64, t26231: f64, t26246: f64, t26251: f64, t26255: f64, t26266: f64, t26268: f64, t28058: f64, t28061: f64, t28063: f64, t28065: f64, t28068: f64, t28070: f64, t28074: f64, t28078: f64, t28080: f64, t24049: f64, t24050: f64, t24058: f64, t24060: f64, t24061: f64, t26272: f64, t26295: f64, t28085: f64, t28089: f64, t28091: f64, t28093: f64, t28095: f64, t28097: f64, t28102: f64, t28104: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29211, t29214, t29219, t29222, t29241) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1728(t2039, t6287, t2075, t5493, t1774, t7801, t19596, t2095, t1268, t1458, t19451, t27188, t28002, t28007, t28943, t28951, t28959, t4028, t7042, t7676);
        let (t29243, t29247, t29252, t29274) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1729(t20085, t2095, t24432, t28830, t23957, t28826, t26231, t26246, t26251, t26255, t26266, t26268, t28058, t28061, t28063, t28065, t28068, t28070, t28074, t28078, t28080);
        let t29285 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1730(t24049, t24050, t24058, t24060, t24061, t26272, t26295, t28085, t28089, t28091, t28093, t28095, t28097, t28102, t28104);
    (t29211, t29214, t29219, t29222, t29241, t29243, t29247, t29252, t29274, t29285)
}
