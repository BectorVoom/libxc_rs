//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta177 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk857;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk858;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk859;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta177(t3014: f64, t972: f64, t4732: f64, t981: f64, t2848: f64, t3037: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t341: f64, t1646: f64, t993: f64, t378: f64, t1647: f64, t1651: f64, t999: f64, t996: f64, t1096: f64, t1079: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4733, t4734, t4736, t4742, t4743) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk857(t3014, t972, t4732, t981, t2848, t3037, t4571, t4576, t4581, t4585, t341);
        let t4746 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk858(t1646, t993);
        let (t4747, t4752, t4757) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk859(t378, t4746, t1647, t1651, t999);
        let (t4758, t4764, t4772) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk860(t4757, t996, t1096, t1651, t1079, t2848, t3070, t4571, t4576, t4581, t4585);
    (t4733, t4734, t4736, t4742, t4743, t4746, t4747, t4752, t4757, t4758, t4764, t4772)
}
