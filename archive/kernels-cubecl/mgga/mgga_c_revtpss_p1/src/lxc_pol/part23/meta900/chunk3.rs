//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2864/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864<F: Float>(t1555: F, t18586: F, t18592: F, t18600: F, t18603: F, t18609: F, t225: F, t229: F, t231: F, t23227: F, t4409: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t73: F, t76943: F, t76961: F, t76975: F, t76981: F, t77001: F, t77016: F, t77033: F, t77061: F, t77118: F, t833: F) -> F {
    let t77120 = (-(t76943 + t76961 + t76975 + t76981 + t77001 + t77016 + t77033 + t77061) * t225 * t229 + F::cast_from(3.0_f64) * t23227 * t833 + F::cast_from(9.0_f64) * t18586 * t1555 - F::cast_from(36.0_f64) * t6006 * t73 * t4417 + F::cast_from(9.0_f64) * t6006 * t4420 - F::cast_from(36.0_f64) * t4409 * t6010 + F::cast_from(180.0_f64) * t18592 * t18600 - F::cast_from(72.0_f64) * t18592 * t18603 + F::cast_from(9.0_f64) * t4409 * t6013 - F::cast_from(36.0_f64) * t18592 * t18609 + t77118) * t231;
    t77120
}
