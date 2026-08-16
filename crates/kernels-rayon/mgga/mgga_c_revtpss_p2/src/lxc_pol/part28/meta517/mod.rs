//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta517(t7810: f64, t999: f64, t7145: f64, t1976: f64, t4746: f64, t1096: f64, t7821: f64, t7160: f64, t4772: f64, t1982: f64, t4930: f64, t1000: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1986: f64, t25634: f64, t25658: f64, t25692: f64, t25695: f64, t4743: f64, t4764: f64, t4773: f64, t4941: f64, t5016: f64, t7102: f64, t7137: f64, t7140: f64, t7151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27556, t27557, t27568, t27575, t27576, t27579, t27580, t27587, t27592) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1936(t7810, t999, t7145, t1976, t4746, t1096, t7821, t7160, t4772, t1982, t4930, t1000, t1647, t1652, t1696, t1978, t1986, t25634, t25658, t25692, t25695, t4743, t4764, t4773, t4941, t5016, t7102, t7137, t7140, t7151);
    (t27556, t27557, t27568, t27575, t27576, t27579, t27580, t27587, t27592)
}
