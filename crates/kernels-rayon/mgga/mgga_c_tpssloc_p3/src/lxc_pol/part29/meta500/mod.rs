//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1856;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta500(t23122: f64, t25064: f64, t4166: f64, t6620: f64, t849: f64, t1516: f64, t23127: f64, t4261: f64, t6621: f64, t23133: f64, t7503: f64, t838: f64, t23046: f64, t242: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25065, t25068, t25069, t25071, t25073, t25077, t25080) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1856(t23122, t25064, t4166, t6620, t849, t1516, t23127, t4261, t6621, t23133, t7503, t838);
        let (t25083, t25084) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1857(t23046, t242, t812);
    (t25065, t25068, t25069, t25071, t25073, t25077, t25080, t25083, t25084)
}
