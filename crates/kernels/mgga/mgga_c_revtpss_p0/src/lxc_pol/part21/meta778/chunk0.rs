//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2770/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770<F: Float>(t50893: F, t162: F, t40188: F, t14331: F, t40186: F, t40203: F, t40205: F, t14362: F, t9572: F, t37: F, t4391: F, t2612: F) -> (F, F, F, F, F, F, F) {
    let t50894 = F::cast_from(0.10389515463408878255e3_f64) * t50893;
    let t50895 = t40188 * t162;
    let t50897 = F::new(72.0) * t50895 * t14331;
    let t50898 = F::new(36.0) * t40186;
    let t50899 = F::cast_from(0.35089341735807877242e1_f64) * t40203;
    let t50900 = F::cast_from(0.10526802520742363173e2_f64) * t40205;
    let t50901 = t14362 * t9572;
    let t50902 = F::cast_from(0.32530743900905219526e-1_f64) * t50901;
    let t50903 = t37 * t4391;
    let t50905 = F::new(36.0) * t50903 * t2612;
    (t50894, t50897, t50898, t50899, t50900, t50902, t50905)
}
