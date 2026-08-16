//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2038;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta575(t11858: f64, t27492: f64, t11926: f64, t25516: f64, t3114: f64, t93596: f64, t25577: f64, t3111: f64, t1020: f64, t25576: f64, t25490: f64, t3215: f64, t11951: f64, t7117: f64, t11643: f64, t25522: f64, t12009: f64, t25505: f64, t25531: f64, t800: f64, t25539: f64, t3244: f64, t11880: f64, t7111: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93658, t93667, t93670, t93673, t93675, t93683) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2038(t11858, t27492, t11926, t25516, t3114, t93596, t25577, t3111, t1020, t25576, t25490, t3215);
        let (t93685, t93687, t93689, t93691, t93694, t93696) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2039(t11951, t7117, t11643, t25522, t12009, t25505, t25531, t800, t25539, t3244, t11880, t7111);
    (t93658, t93667, t93670, t93673, t93675, t93683, t93685, t93687, t93689, t93691, t93694, t93696)
}
