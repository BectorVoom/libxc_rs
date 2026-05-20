//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1642/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642<F: Float>(t23467: F, t52508: F, t6109: F, t11385: F, t2926: F, t23568: F, t4719: F, t23649: F, t18898: F, t64043: F, t981: F, t1699: F, t5023: F, t78478: F, t88004: F, t88007: F, t88012: F, t88016: F, t88023: F, t88026: F, t88028: F) -> (F, F, F, F, F, F, F) {
    let t88030 = F::cast_from(0.3859675079686208416e3_f64) * t52508 * t23467;
    let t88031 = t6109 * t6109;
    let t88034 = F::cast_from(0.57895126195293126241e3_f64) * t11385 * t88031 * t2926;
    let t88036 = F::cast_from(0.20779030926817756511e3_f64) * t4719 * t23568;
    let t88038 = F::cast_from(0.4101607543286562663e4_f64) * t4719 * t23649;
    let t88041 = F::cast_from(0.61524113149298439947e4_f64) * t981 * t18898 * t64043;
    let t88042 = -F::new(4.0) * t1699 * t5023 * t78478 - t88004 + t88007 - t88012 + t88016 - t88023 + t88026 - t88028 - t88030 + t88034 - t88036 - t88038 - t88041;
    (t88030, t88031, t88034, t88036, t88038, t88041, t88042)
}
