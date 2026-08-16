//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta521<F: Float>(t1984: F, t27543: F, t359: F, t1646: F, t7135: F, t7145: F, t7828: F, t999: F, t7160: F, t1651: F, t7821: F, t1096: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27676, t27679, t27680, t27683, t27684, t27687, t27688, t27691, t27692, t27695) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1943::<F>(t1984, t27543, t359, t1646, t7135, t7145, t7828, t999, t7160, t1651, t7821, t1096);
    (t27676, t27679, t27680, t27683, t27684, t27687, t27688, t27691, t27692, t27695)
}
