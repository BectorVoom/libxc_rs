//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2198;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2199;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta632<F: Float>(t101435: F, t2035: F, t28196: F, t28197: F, t75365: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t2340: F, t94982: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F, t94974: F, t94979: F, t94981: F, t114: F, t508: F, t651: F, t530: F, t7933: F, t2014: F, t25865: F, t1353: F, t22496: F, t28167: F, t8717: F, t25082: F, t73394: F) -> (F, F, F, F, F, F, F) {
        let (t101436, t101439, t101448, t101451, t101454, t101455) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2198::<F>(t101435, t2035, t28196, t28197, t75365, t94976, t1513, t94975, t28036, t94978, t25823, t4287);
        let t101468 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2199::<F>(t101455, t1513, t2340, t94982, t4287, t665, t25826, t2366, t13509, t6998, t101448, t101451, t101454, t94974, t94979, t94981);
        let (t101469, t101472, t101476, t101482, t101485) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200::<F>(t114, t101468, t508, t651, t530, t7933, t2014, t25865, t1353, t22496, t28167, t8717, t25082, t73394);
    (t101436, t101439, t101469, t101472, t101476, t101482, t101485)
}
