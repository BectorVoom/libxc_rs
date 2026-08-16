//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta818(t19676: f64, t3127: f64, t3172: f64, t16158: f64, t4834: f64, t19791: f64, t19781: f64, t3091: f64, t43131: f64, t19939: f64, t11262: f64, t3161: f64, t6311: f64, t11274: f64, t20029: f64, t11656: f64, t19920: f64, t6262: f64, t15817: f64, t4820: f64, t15775: f64, t1032: f64, t1040: f64, t19856: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65527, t65538, t65553, t65567, t65570, t65581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665(t19676, t3127, t3172, t16158, t4834, t19791, t19781, t3091, t43131, t19939, t11262, t3161, t6311);
        let (t65585, t65589, t65596, t65598, t65610, t65613) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666(t11274, t20029, t11656, t19920, t11262, t3127, t6262, t15817, t4820, t15775, t4834, t1032, t1040, t19856);
    (t65527, t65538, t65553, t65567, t65570, t65581, t65585, t65589, t65596, t65598, t65610, t65613)
}
