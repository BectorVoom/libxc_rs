//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3314/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3314(t231: f64, t2782: f64, t2783: f64, t62803: f64, t18689: f64, t2435: f64, t18688: f64, t2439: f64, t2777: f64, t14587: f64, t51548: f64, t10943: f64, t14546: f64, t18525: f64, t18677: f64, t18699: f64, t2646: f64, t40284: f64, t40303: f64, t40314: f64, t40316: f64, t40318: f64, t4504: f64, t4514: f64, t51512: f64, t62760: f64) -> f64 {
    let t62840 = t2782 * t2783 * t62803 * t231;
    let t62843 = t2435 * t18689;
    let t62847 = t2439 * t2777 * t18688;
    let t62853 = t2782 * t51548 * t14587;
    let t62856 = -0.79025390195226139182e1_f64 * t14546 * t62760 * t18525 - 0.65854491829355115987e0_f64 * t4514 * t18699 * t2646 + 0.39512695097613069591e1_f64 * t4504 * t18677 * t10943 + 0.21951497276451705328e-1_f64 * t62840 - 0.73171657588172351096e-2_f64 * t40303 + 0.73171657588172351096e-2_f64 * t62843 - t40314 + t40316 + 0.52039682876708176102e-1_f64 * t51512 - 0.65049603595885220126e-3_f64 * t62847 - 0.39512695097613069591e1_f64 * t14546 * t18677 * t40284 - 0.43902994552903410656e-1_f64 * t62853 + 0.22089088168956307394e-3_f64 * t40318;
    t62856
}
