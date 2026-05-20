//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1821;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1822;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1823;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta480<F: Float>(t3244: F, t7111: F, t3111: F, t7132: F, t1971: F, t3229: F, t351: F, t1058: F, t7126: F, t1973: F, t3201: F, t1020: F, t7125: F, t7114: F, t1972: F, t3196: F, t7131: F, t3104: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25543, t25551, t25553, t25554, t25557, t25560, t25561) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1821::<F>(t3244, t7111, t3111, t7132, t1971, t3229, t351, t1058, t7126, t1973, t3201, t1020, t7125);
        let (t25564, t25566, t25569) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1822::<F>(t1058, t7114, t1972, t3196, t1020, t7131);
        let t25576 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1823::<F>(t1971, t3104);
        let t25577 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1824::<F>(t25576, t351);
    (t25543, t25551, t25553, t25554, t25557, t25560, t25561, t25564, t25566, t25569, t25576, t25577)
}
