//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1206;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta370(t14328: f64, t14334: f64, t14336: f64, t14339: f64, t5819: f64, t750: f64, t2611: f64, t2398: f64, t5999: f64, t5825: f64, t706: f64, t4305: f64, t4311: f64, t14363: f64, t162: f64, t18298: f64, t187: f64, t10563: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18547) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1206(t14328, t14334, t14336, t14339, t5819, t750, t2611, t2398, t5999, t5825, t706, t4305, t4311);
        let (t18548, t18549, t18552, t18553) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1207(t18547, t14363, t162, t18298, t187, t10563, t14324, t14343, t14345, t14372, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t9394);
    (t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t18553)
}
