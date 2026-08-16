//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta419(t14328: f64, t14334: f64, t14336: f64, t14339: f64, t5819: f64, t750: f64, t2611: f64, t2398: f64, t5999: f64, t5825: f64, t706: f64, t4305: f64, t4311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18535, t18536, t18537, t18538, t18539, t18540, t18541, t18543, t18544, t18545, t18546, t18547) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1803(t14328, t14334, t14336, t14339, t5819, t750, t2611, t2398, t5999, t5825, t706, t4305, t4311);
    (t18535, t18536, t18537, t18538, t18539, t18540, t18541, t18543, t18544, t18545, t18546, t18547)
}
