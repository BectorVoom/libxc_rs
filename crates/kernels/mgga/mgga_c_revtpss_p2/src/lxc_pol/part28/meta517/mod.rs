//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta517<F: Float>(t7810: F, t999: F, t7145: F, t1976: F, t4746: F, t1096: F, t7821: F, t7160: F, t4772: F, t1982: F, t4930: F, t1000: F, t1647: F, t1652: F, t1696: F, t1978: F, t1986: F, t25634: F, t25658: F, t25692: F, t25695: F, t4743: F, t4764: F, t4773: F, t4941: F, t5016: F, t7102: F, t7137: F, t7140: F, t7151: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27556, t27557, t27568, t27575, t27576, t27579, t27580, t27587, t27592) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1936::<F>(t7810, t999, t7145, t1976, t4746, t1096, t7821, t7160, t4772, t1982, t4930, t1000, t1647, t1652, t1696, t1978, t1986, t25634, t25658, t25692, t25695, t4743, t4764, t4773, t4941, t5016, t7102, t7137, t7140, t7151);
    (t27556, t27557, t27568, t27575, t27576, t27579, t27580, t27587, t27592)
}
