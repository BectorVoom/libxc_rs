//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1093/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1093(t34937: f64, t34941: f64, t34946: f64, t34949: f64, t34953: f64, t34958: f64, t34962: f64, t34965: f64, t34969: f64, t34973: f64, t34977: f64, t34980: f64, t34984: f64, t34986: f64, t34987: f64, t34991: f64, t34994: f64, t34996: f64) -> f64 {
    let t34998 = 0.62896184579208304136e-3_f64 * t34937 - 0.94344276868812456204e-2_f64 * t34941 - t34946 + 0.42874018118069736972e-3_f64 * t34949 + 0.21437009059034868486e-3_f64 * t34953 + t34958 - t34962 - 0.42874018118069736972e-3_f64 * t34965 + 0.15724046144802076034e-2_f64 * t34969 - 0.62896184579208304136e-3_f64 * t34973 - 0.10718504529517434243e-2_f64 * t34977 + 0.10718504529517434243e-2_f64 * t34980 - 0.64311027177104605458e-2_f64 * t34984 - t34986 - t34987 + t34991 - 0.12862205435420921092e-1_f64 * t34994 - t34996 / 48.0_f64;
    t34998
}
