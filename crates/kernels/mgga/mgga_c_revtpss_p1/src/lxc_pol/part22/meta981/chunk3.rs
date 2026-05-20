//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3314/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3314<F: Float>(t231: F, t2782: F, t2783: F, t62803: F, t18689: F, t2435: F, t18688: F, t2439: F, t2777: F, t14587: F, t51548: F, t10943: F, t14546: F, t18525: F, t18677: F, t18699: F, t2646: F, t40284: F, t40303: F, t40314: F, t40316: F, t40318: F, t4504: F, t4514: F, t51512: F, t62760: F) -> F {
    let t62840 = t2782 * t2783 * t62803 * t231;
    let t62843 = t2435 * t18689;
    let t62847 = t2439 * t2777 * t18688;
    let t62853 = t2782 * t51548 * t14587;
    let t62856 = -F::cast_from(0.79025390195226139182e1_f64) * t14546 * t62760 * t18525 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18699 * t2646 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t18677 * t10943 + F::cast_from(0.21951497276451705328e-1_f64) * t62840 - F::cast_from(0.73171657588172351096e-2_f64) * t40303 + F::cast_from(0.73171657588172351096e-2_f64) * t62843 - t40314 + t40316 + F::cast_from(0.52039682876708176102e-1_f64) * t51512 - F::cast_from(0.65049603595885220126e-3_f64) * t62847 - F::cast_from(0.39512695097613069591e1_f64) * t14546 * t18677 * t40284 - F::cast_from(0.43902994552903410656e-1_f64) * t62853 + F::cast_from(0.22089088168956307394e-3_f64) * t40318;
    t62856
}
