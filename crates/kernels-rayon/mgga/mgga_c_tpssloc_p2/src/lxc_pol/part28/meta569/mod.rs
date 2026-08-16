//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1848;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta569(t252: f64, t4119: f64, t22986: f64, t6646: f64, t829: f64, t22690: f64, t7520: f64, t81573: f64, t25249: f64, t2684: f64, t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t13384: f64, t2647: f64, t22893: f64, t23164: f64, t25306: f64, t25236: f64, t13381: f64, t1888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87130, t87133, t87140, t87150, t87153) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1848(t252, t4119, t22986, t6646, t829, t22690, t7520, t81573, t25249, t2684, t25324, t6562, t794);
        let (t87155, t87159, t87165, t87171, t87174) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1849(t23030, t25258, t13384, t22986, t2647, t6646, t22893, t23164, t25306, t25236, t13381, t1888);
    (t87130, t87133, t87140, t87150, t87153, t87155, t87159, t87165, t87171, t87174)
}
