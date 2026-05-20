//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2038;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta575<F: Float>(t11858: F, t27492: F, t11926: F, t25516: F, t3114: F, t93596: F, t25577: F, t3111: F, t1020: F, t25576: F, t25490: F, t3215: F, t11951: F, t7117: F, t11643: F, t25522: F, t12009: F, t25505: F, t25531: F, t800: F, t25539: F, t3244: F, t11880: F, t7111: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93658, t93667, t93670, t93673, t93675, t93683) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2038::<F>(t11858, t27492, t11926, t25516, t3114, t93596, t25577, t3111, t1020, t25576, t25490, t3215);
        let (t93685, t93687, t93689, t93691, t93694, t93696) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2039::<F>(t11951, t7117, t11643, t25522, t12009, t25505, t25531, t800, t25539, t3244, t11880, t7111);
    (t93658, t93667, t93670, t93673, t93675, t93683, t93685, t93687, t93689, t93691, t93694, t93696)
}
