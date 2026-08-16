//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2084;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta611(t23562: f64, t343: f64, t82916: f64, t3008: f64, t40: f64, t23482: f64, t3: f64, t23563: f64, t23514: f64, t3128: f64, t82895: f64, t23471: f64, t23473: f64, t1933: f64, t23479: f64, t23433: f64, t3103: f64, t10889: f64, t23535: f64, t3033: f64, t10908: f64, t6755: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82918, t82921, t82923, t82926, t82927, t82941, t82943) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2084(t23562, t343, t82916, t3008, t40, t23482, t3, t23563, t23514, t3128, t82895, t23471);
        let (t82944, t82951, t82953, t82956, t82961) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2085(t23473, t82943, t1933, t23479, t82921, t23433, t3103, t10889, t23535, t3033, t10908, t6755);
    (t82918, t82923, t82926, t82927, t82941, t82943, t82944, t82951, t82953, t82956, t82961)
}
